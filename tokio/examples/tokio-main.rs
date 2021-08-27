// #[tokio::main]
// async fn main() {
//     println!("hello");
// }

// is the same as 
fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        println!("hello");
    })
}