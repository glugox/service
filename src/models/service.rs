use crate::auth::Auth;
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};

type Url = String;

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct Service {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub active: Option<bool>,
}

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct NewService {
    pub name: String,
    pub url: String,
    pub active: Option<bool>,
}

#[derive(Queryable, Serialize)]
pub struct ServiceList {
    pub services: Vec<Service>,
}


impl Service {
  pub fn before_insert(&self) -> Service {
       
     Service {
         id: self.id,
         name: self.name.clone(),
         url: self.url.clone(),
         active: self.active.clone()
     }
   }
}

