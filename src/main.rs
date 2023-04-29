use goridge_rs::frame::frame_flags::Flag::CodecProto;
use goridge_rs::frame::Frame;
use std::{
    io::{Read, Write},
    net::TcpStream,
};

use crate::rpc::Request;

mod generated;
pub mod rpc;
// use yew::prelude::*;
//
// #[function_component]
// fn App() -> Html {
//     let counter = use_state(|| 0);
//     let onclick = {
//         let counter = counter.clone();
//         move |_| {
//             let value = *counter + 1;
//             counter.set(value);
//         }
//     };
//
//     html!(
//     <div>
//     <button {onclick}> {"+1"} </button>
//     <p> {*counter}</p>
//     </div>
//     )
// }

fn main() -> std::io::Result<()> {
    let v = rpc::Method::Version;
    println!("{:?}", v.to_string());

    let mut msg = generated::InMsg::default();
    msg.payload = String::from("foo from Rust!!");

    let m = protobuf::Message::write_to_bytes(&msg).unwrap();

    let mut rpc = rpc::RPC::new("").unwrap();
    rpc.send(method, payload);
    

    // yew::Renderer::<App>::new().render();
    //
    //

    // let mut payload: Vec<u8> = "test.TestMethod".into();

    Ok(())
}

/*

        opts := c.frame.ReadOptions(c.frame.Header())
        if len(opts) != 2 {
            return errors.E(op, errors.Str("should be 2 options. SEQ_ID and METHOD_LEN"))
        }
        payload := c.frame.Payload()[opts[1]:]
        if len(payload) == 0 {
            return nil
        }

        // check if the out message is a correct proto.Message
        // instead send an error
        if pOut, ok := out.(proto.Message); ok {
            err := proto.Unmarshal(payload, pOut)
            if err != nil {
                return errors.E(op, err)
            }
            return nil
        }

        return errors.E(op, errors.Str("message type is not a proto"))
*/
