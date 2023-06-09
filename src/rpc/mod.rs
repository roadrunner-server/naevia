use goridge_rs::frame::frame_flags::Flag::CodecProto;
use goridge_rs::frame::Frame;
use std::io::{Read, Write};

mod methods;

#[derive(Debug)]
pub enum Method {
    Version,
    Config,
}

impl std::fmt::Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Config => f.write_str(methods::CONFIG_RPC),
            Self::Version => f.write_str(methods::VERSION_RPC),
        }
    }
}

impl Into<Vec<u8>> for Method {
    fn into(self) -> Vec<u8> {
        match self {
            Self::Version => {
                return Vec::from(methods::VERSION_RPC);
            }
            Self::Config => {
                return Vec::from(methods::CONFIG_RPC);
            }
        }
    }
}

// TODO (is that fine??)
pub trait Request {
    fn send(&mut self, method: Method, payload: &Vec<u8>) -> Result<Vec<u8>, std::io::Error>;
}

pub struct RPC {
    client: std::net::TcpStream,
    seq_id: u32,
}

impl RPC {
    pub fn new(addr: &str) -> Result<RPC, std::io::Error> {
        let client = std::net::TcpStream::connect(addr)?;

        Ok(RPC { client, seq_id: 1 })
    }
}

impl Request for RPC {
    fn send(&mut self, method: Method, payload: &Vec<u8>) -> Result<Vec<u8>, std::io::Error> {
        // init frame + add a version
        let mut frame = Frame::default();
        let mut mtd = <Method as Into<Vec<u8>>>::into(method);
        frame.write_version(1);
        frame.write_flags(&[CodecProto]);
        frame.write_options(&[self.seq_id, mtd.len() as u32]);
        if self.seq_id >= u32::MAX {
            self.seq_id = 1;
        }

        self.seq_id += 1;

        mtd.extend_from_slice(payload);
        frame.write_payload(mtd);
        frame.write_crc();

        // Send the request payload to the server
        self.client.write_all(&frame.bytes())?;

        // // Read the response from the server
        let mut read_f = Frame::default();
        let _ = self.client.read_exact(read_f.header_mut())?;

        if read_f.verify_crc().is_err() {
            panic!("incorrect CRC sum!");
        }

        let hl = read_f.read_hl();
        if hl > 3 {
            let mut b = vec![0; ((hl - 3) * 4) as usize];
            self.client.read_exact(&mut b)?;
            read_f.extend_header(&b);
        }

        let pl = read_f.read_payload_len();
        let mut b_pld = vec![0; pl as usize];
        self.client.read_exact(&mut b_pld)?;

        match read_f.read_options() {
            Some(opts) => {
                let pld_off = opts[1] as usize;
                Ok(b_pld[pld_off..].to_vec())
            }
            None => {
                panic!("no options provided")
            }
        }
    }
}

mod tests {
    #![allow(unused_imports)]
    use super::RPC;
    use crate::generated;
    use crate::rpc::{Request, Method};

    #[test]
    fn test() {
        let mut msg = generated::InMsg::default();
        msg.payload = String::from("foo from");
        let m = protobuf::Message::write_to_bytes(&msg).unwrap();

        let mut rpc = RPC::new("127.0.0.1:8999").unwrap();
        let resp = rpc.send(Method::Version, &m).unwrap();
        println!("response: {}", std::str::from_utf8(&resp.to_vec()).unwrap());
    }
}
