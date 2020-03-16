use crate::auth::Auth;
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use crate::schema::services;
use crate::db;
use crate::db::Conn;
use validator::Validate;

type Url = String;

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct Service {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub active: Option<bool>,
}



#[derive(Queryable, Debug, Serialize, Deserialize, Insertable, Validate)]
#[table_name = "services"]
pub struct NewServiceData {
    #[validate(length(min = 1))]
    pub(crate) name: String,
    pub(crate) url: String,
    pub(crate) active: Option<bool>,
}

#[derive(Deserialize)]
pub struct UpdateService {
    pub(crate) service: db::services::UpdateServiceData,
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

