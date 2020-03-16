use rocket::State;
use rocket_contrib::json::{Json, JsonValue};
use validator::Validate;

use crate::auth::{ApiKey, Auth};
use crate::config::AppState;
use crate::db::{self, services::ServiceCreationError};
use crate::errors::{Errors, FieldValidator};
use crate::models::service::{UpdateService, NewServiceData};


#[post("/", format = "json", data = "<new_service>")]
pub fn post_service(
    new_service: Json<NewServiceData>,
    conn: db::Conn,
    state: State<AppState>,
) -> Result<JsonValue, Errors> {
    let new_service = new_service.into_inner();

    let mut extractor = FieldValidator::validate(&new_service);
    let name = new_service.name;
    let url =new_service.url;
    let active = new_service.active;

    extractor.check()?;

    db::services::create(&conn, name.clone(), url.clone(), active)
        .map(|service| json!({ "service": service.before_insert() }))
        .map_err(|error| {
            let field = match error {
                ServiceCreationError::DuplicatedServiceName => "name",
            };
            Errors::new(&[(field, "has already been taken")])
        })
}

#[get("/")]
pub fn get_services(_key: ApiKey, conn: db::Conn) -> Option<JsonValue> {
    db::services::find(&conn).map(|service| json!(service))
}

#[get("/<id>")]
pub fn get_service(_key: ApiKey, id: i32, conn: db::Conn) -> Option<JsonValue> {
    db::services::find_one(&conn, id).map(|service| json!({ "service": service }))
}


#[put("/", format = "json", data = "<service>")]
pub fn put_service(
    service: Json<UpdateService>,
    auth: Auth,
    conn: db::Conn,
    state: State<AppState>,
) -> Option<JsonValue> {
    db::services::update(&conn, auth.id, &service.service)
        .map(|service| json!({ "service": service.before_insert() }))
}

#[delete("/<id>")]
pub fn delete_service(id: i32, _auth: Auth, conn: db::Conn) {
    db::services::delete(&conn, id);
}
