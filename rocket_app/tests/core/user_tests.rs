use reqwest::StatusCode;
use std::collections::HashMap;

use rocket;
use rocket::http::{ContentType, Status};
use rocket::local::Client;

#[test]
fn test_user_register() {
    let _ = env_logger::try_init();
    ::std::env::set_var("RUST_LOG", "info,cargo=WARN,fina_app=DEBUG");
    ::std::env::set_var("RUST_BACKTRACE", "full");
    ::std::env::set_var(
        "DATABASE_URL",
        "postgres://billac:billac@localhost/billacdb",
    );
    let (rocket, db) = fina_app_lib::rocket();
    let client = Client::new(rocket).expect("Rocket client");

    // Check that a message with ID 1 doesn't exist.
    let res = client
        .get("/message/1")
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(res.status(), Status::NotFound);

    // Add a new message with ID 1.
    let res = client
        .post("/api/user/signup")
        .header(ContentType::JSON)
        .body(r#"{
            "first_name": "John",
            "last_name": "Doe",
            "email": "john.doe@acme.org",
            "password": "h@rdToGu3$s"
         }"#)
        .dispatch();

    assert_eq!(res.status(), Status::Ok);

    // Check that the message exists with the correct contents.
    let mut res = client
        .get("/message/1")
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(res.status(), Status::Ok);
    let body = res.body().unwrap().into_string().unwrap();
    assert!(body.contains("Hello, world!"));
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
