use jsonrpc_http_server::jsonrpc_core::{IoHandler, Value, Params};
use jsonrpc_http_server::ServerBuilder;
use jsonrpc_client_transports::RpcResult;

// fn main() {
// 	let mut io = IoHandler::default();
// 	io.add_method("say_hello", |_params: Params| async {
// 		Ok(Value::String("hello".to_owned()))
// 	});

// 	let server = ServerBuilder::new(io)
// 		.threads(3)
// 		.start_http(&"127.0.0.1:3030".parse().unwrap())
// 		.unwrap();
// 	server.wait();
// }

use jsonrpc_core::Result;
use jsonrpc_derive::rpc;
use futures::{future, Future, FutureExt, StreamExt, TryFutureExt};

#[rpc]
pub trait Rpc {
	/// Adds two numbers and returns a result
	#[rpc(name = "add")]
	fn add(&self, a: u64, b: u64) -> Result<u64>;

	#[rpc(name = "hello")]
	fn hello(&self, msg: String) -> Result<String>;
}

pub struct RpcImpl;
impl Rpc for RpcImpl {
	fn hello(&self, msg: String) -> Result<String> {
        Ok("hello hello 123".to_owned())
    }

	fn add(&self, a: u64, b: u64) -> Result<u64> {
		Ok(a + b)
	}
}

fn main() {
	let mut io = jsonrpc_core::IoHandler::new();
	io.extend_with(RpcImpl.to_delegate());

	let server = ServerBuilder::new(io)
		.threads(3)
		.start_http(&"127.0.0.1:3030".parse().unwrap())
		.unwrap();

	server.wait();
}
