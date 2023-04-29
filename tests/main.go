package main

import (
	"net"
	"net/rpc"

	goridgeRpc "github.com/roadrunner-server/goridge/v3/pkg/rpc"
)

type RPC struct{}

func (r *RPC) TestMethod(in *InMsg, out *OutMsg) error {
	println("hello from go!")
	out.Payload = "foo from Go!"
	return nil
}

func main() {
	r := new(RPC)

	err1 := rpc.RegisterName("test", r)
	if err1 != nil {
		panic(err1)
	}

	listener, err := net.Listen("tcp", "127.0.0.1:8999")
	if err != nil {
		panic(err)
	}

	for {
		conn, err := listener.Accept()
		if err != nil {
			panic(err)
		}

		go rpc.ServeCodec(goridgeRpc.NewCodec(conn))
	}
}
