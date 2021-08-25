#[macro_use] 
extern crate jsonrpc_client_core;
extern crate jsonrpc_client_http;

use jsonrpc_client_http::HttpTransport;
use serde_json::Value;

jsonrpc_client!(pub struct AccountServiceClient {
    pub fn findById(&mut self, account_id: &str) -> RpcResult<Value>;
});


fn main() {
    let transport = HttpTransport::new().standalone().unwrap();
    let transport_handle = transport.handle("http://localhost:20097/account-api").unwrap();
    let mut client = AccountServiceClient::new(transport_handle);
    let result1 = client.findById("a4c20568-2eb5-4ad4-bf6c-ea13707f6f09").call().unwrap();

    println!("{}", result1);
}
