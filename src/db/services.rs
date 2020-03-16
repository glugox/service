use crate::db::Conn;
use crate::models::service::*;
use crate::schema::services;
use std::ops::Deref;

use crypto::scrypt::{scrypt_check, scrypt_simple, ScryptParams};
use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error};
use serde::Deserialize;

#[derive(Insertable)]
#[table_name = "services"]
pub struct NewService<'a> {
    pub name: &'a str,
    pub url: &'a str,
    pub active: bool,
}

pub enum ServiceCreationError {
    DuplicatedServiceName,
}

impl From<Error> for ServiceCreationError {
    fn from(err: Error) -> ServiceCreationError {
        if let Error::DatabaseError(DatabaseErrorKind::UniqueViolation, info) = &err {
            match info.constraint_name() {
                Some("service_name_key") => return ServiceCreationError::DuplicatedServiceName,
                _ => {}
            }
        }
        panic!("Error creating service: {:?}", err)
    }
}

pub fn create(
    conn: &Conn,
    name: &str,
    url: &str,
    active: bool,
) -> Result<Service, ServiceCreationError> {

    let new_service = &NewService {
        name,
        url,
        active
    };

    diesel::insert_into(services::table)
        .values(new_service)
        .get_result::<Service>(conn.deref())
        .map_err(Into::into)
}


/// Return a list of all Services
/// TODO: Pagination
pub fn find(conn: &Conn) -> Option<ServiceList> {

    let services : Vec<Service> = services::table.load::<Service>(conn.deref())
        .map_err(|err| println!("Can not load services!: {}", err))
        .unwrap();

    Some(ServiceList{
        services
    })
}


pub fn find_one(conn: &Conn, id: i32) -> Option<Service> {
    services::table
        .find(id)
        .get_result(conn.deref())
        .map_err(|err| println!("find_service: {}", err))
        .ok()
}

pub fn delete(conn: &Conn, id: i32) {
    let result = diesel::delete(services::table.filter(services::id.eq(id))).execute(conn.deref());
    if let Err(err) = result {
        eprintln!("services::delete: {}", err);
    }
}

// TODO: remove clone when diesel will allow skipping fields
#[derive(Deserialize, AsChangeset, Default, Clone)]
#[table_name = "services"]
pub struct UpdateServiceData {
    name: Option<String>,
}

pub fn update(conn: &Conn, id: i32, data: &UpdateServiceData) -> Option<Service> {
    let data = &UpdateServiceData {
        // Place to set particular fields... ex password: None,
        ..data.clone()
    };
    diesel::update(services::table.find(id))
        .set(data)
        .get_result(conn.deref())
        .ok()
}
