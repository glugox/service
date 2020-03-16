use std::collections::HashMap;
use crate::models::service::{NewService, Service};

pub struct Discovery{}

pub trait HasDiscovery {
    fn register();
}

impl HasDiscovery for Discovery {
    fn register() {
        let _future = send_registration();
    }
}

#[tokio::main]
pub async fn send_registration() -> Result<(), Box<dyn std::error::Error>> {

        let new_service = NewService {
            name: "Dummy001".into(),
            url: "https://google.com".into(),
            active: Some(true),
        };
        let service: Service = reqwest::Client::new()
            .post("http://127.0.0.1:9100")
            .json(&new_service)
            .send()
            .await?
            .json()
            .await?;

        println!("{:#?}", service);
        Ok(())
}




