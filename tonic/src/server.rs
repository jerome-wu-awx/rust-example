use tonic::{transport::Server, Request, Response, Status};

use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest};

pub mod hello_world { 
    tonic::include_proto!("helloworld"); // 这里指定的字符串必须与proto的包名称一致
}

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(&self, request: Request<HelloRequest>,) -> Result<Response<HelloReply>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let reply = hello_world::HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> { 
    let addr = "[::1]:50051".parse()?; 
    let greeter = MyGreeter::default();
    println!("Server listening on [::1]:50051");
    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr).await?;
    Ok(())
}