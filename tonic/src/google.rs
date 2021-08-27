
pub mod api {
    tonic::include_proto!("google.pubsub.v1");
}
use api::{publisher_client::PublisherClient, ListTopicsRequest};

pub mod google {
    #[path = ""]
    pub mod pubsub {
        #[path = "google.pubsub.v1.rs"]
        pub mod v1;
    }
}

fn main() {}