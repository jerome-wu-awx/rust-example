use jsonrpc_client_transports::TypedClient;
use jsonrpc_client_transports::RpcChannel;
use jsonrpc_client_transports::RpcResult;
use jsonrpc_core_client::transports::http;
use tokio::runtime::Runtime;
use futures::{future, Future, FutureExt, StreamExt, TryFutureExt};
use serde_json::Value;

struct MyClient(TypedClient);

impl From<RpcChannel> for MyClient {
    fn from(channel: RpcChannel) -> Self {
        MyClient(channel.into())
    }
}

impl MyClient {
    fn find_by_id(&self, account_id: &'static str) -> impl Future<Output = RpcResult<Value>> {
        self.0.call_method("findById", "String", (account_id,))
    }
}

fn main() {
    let uri = "http://localhost:20097/account-api";

    let run = async {
        let client: MyClient = http::connect(uri).await?;
        let result = client.find_by_id("a4c20568-2eb5-4ad4-bf6c-ea13707f6f09").await?;

        println!("{}", result);
        Ok(()) as RpcResult<_>
    };
    Runtime::new().unwrap().block_on(run).unwrap();
}
