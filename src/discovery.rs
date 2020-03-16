use std::collections::HashMap;
use crate::models::service::{NewServiceData, Service};

#[tokio::main]
pub async fn send_registration( name: String, url: String, active: bool ) -> Result<(), Box<dyn std::error::Error>> {

        let new_service = NewServiceData {
            name,
            url,
            active: Some(active),
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




