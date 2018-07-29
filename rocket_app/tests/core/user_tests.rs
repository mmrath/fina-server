use reqwest::StatusCode;
use std::collections::HashMap;

use common;
use model::core::User;
use rocket;
use rocket::http::{ContentType, Status};
use rocket::local::Client;
use serde_json;
use util;

#[test]
fn test_user_register() {
    let _ = env_logger::try_init();
    ::std::env::set_var("RUST_LOG", "info,cargo=WARN,fina_app=DEBUG");
    ::std::env::set_var("RUST_BACKTRACE", "full");
    ::std::env::set_var(
        "DATABASE_URL",
        "postgres://billac:billac@localhost/billacdb",
    );

    let (rocket, context) = fina_app_lib::rocket();

    common::clean_db((&context.unwrap()).db());

    let client = Client::new(rocket).expect("Rocket client");

    // Add a new message with ID 1.
    let mut res = client
        .post("/api/user/signup")
        .header(ContentType::JSON)
        .body(
            r#"{
            "first_name": "John",
            "last_name": "Doe",
            "email": "john.doe@acme.org",
            "password": "h@rdToGu3$s"
         }"#,
        )
        .dispatch();

    assert_eq!(res.status(), Status::Ok);
    let user: User = serde_json::from_str(&res.body_string().unwrap()).unwrap();

    assert_eq!(user.first_name, "John");
    assert_eq!(user.last_name, "Doe");
    assert_eq!(user.username, "john.doe@acme.org");
    assert_eq!(user.email, "john.doe@acme.org");
    assert_eq!(user.phone_number, None);
    assert_eq!(user.activated, false);
    assert_eq!(user.locked, false);
    assert_eq!(user.failed_logins, 0);

    let get_user_url = format!("/api/user/{}", user.id);

    // Check that the message exists with the correct contents.
    let mut res = client
        .get(get_user_url)
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(res.status(), Status::Ok);
    let fetched_user: User = serde_json::from_str(&res.body_string().unwrap()).unwrap();
    assert_eq!(user, fetched_user);

    let mut res = client
        .post("/api/user/login")
        .header(ContentType::JSON)
        .body(
            r#"{
            "username": "john.doe@acme.org",
            "password": "h@rdToGu3$s"
         }"#,
        )
        .dispatch();

    let resp_body: HashMap<String, String> =
        serde_json::from_str(&res.body_string().unwrap()).unwrap();

    assert_eq!(res.status(), Status::BadRequest);
    assert_eq!(resp_body.get("error").unwrap(), "LoginError");
    assert_eq!(resp_body.get("kind").unwrap(), "AccountNotYetActivated");
}

/*
fn test_find_user() {
    let _ = env_logger::try_init();
    ::std::env::set_var("RUST_LOG", "info,cargo=WARN,fina_app=DEBUG");
    ::std::env::set_var("RUST_BACKTRACE", "full");

    let conn = utils::connect_and_reset();
    let client = reqwest::Client::new();

    let mut json: HashMap<&str, &str> = HashMap::new();
    json.insert("first_name", "This is a sentence.");
    json.insert("last_name", "test");
    json.insert("login", "test");
    json.insert("email", "test");

    let response = client
        .post(&format!("{}/user/1", "http://localhost:8080/api"))
        .json(&json)
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::Ok);
}
*/
