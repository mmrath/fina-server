use std::io::Cursor;

use rocket::http::{ContentType, Status};
use rocket::request::Request;
use rocket::response::{Responder, Response};
use rocket_contrib::JsonValue;
use serde::Serialize;
use util;

#[derive(Debug)]
pub struct ApiError {
    data: JsonValue,
    status: Status,
}

impl ApiError {


    /// Set the data of the `Response` to `data`.
    pub fn data<T: Serialize>(mut self, data: T) -> ApiError {
        self.data = json!(&data);
        self
    }

    /// Convenience method to set `self.data` to `{"message": message}`.
    pub fn message(mut self, message: &str) -> ApiError {
        self.data = json!({
            "message": message
        });
        self
    }

}


impl<'r> Responder<'r> for ApiError {
    fn respond_to(self, _req: &Request) -> Result<Response<'r>, Status> {
        let body = self.data;

        Response::build()
            .status(self.status)
            .sized_body(Cursor::new(body.to_string()))
            .header(ContentType::JSON)
            .ok()
    }
}

impl<T: Serialize + util::error::Error + std::fmt::Debug> From<T> for ApiError {
    fn from(error: T) -> ApiError {

        let status = if error.is_internal_err() {
            error!("{:?}", error);
            Status::InternalServerError

        }else { Status::BadRequest };

        Self { status, data: json!(&error)}
    }
}

pub fn ok() -> ApiError {
    ApiError {
        data: json!(null),
        status: Status::Ok,
    }
}

pub fn created() -> ApiError {
    ApiError {
        data: json!(null),
        status: Status::Created,
    }
}

pub fn accepted() -> ApiError {
    ApiError {
        data: json!(null),
        status: Status::Accepted,
    }
}

pub fn no_content() -> ApiError {
    ApiError {
        data: json!(null),
        status: Status::NoContent,
    }
}

pub fn bad_request() -> ApiError {
    ApiError {
        data: json!(null),
        status: Status::BadRequest,
    }
}

pub fn unauthorized() -> ApiError {
    ApiError {
        data: json!(null),
        status: Status::Unauthorized,
    }
}

pub fn forbidden() -> ApiError {
    ApiError {
        data: json!(null),
        status: Status::Forbidden,
    }
}

pub fn not_found() -> ApiError {
    ApiError {
        data: json!(null),
        status: Status::NotFound,
    }
}

pub fn method_not_allowed() -> ApiError {
    ApiError {
        data: json!(null),
        status: Status::MethodNotAllowed,
    }
}

pub fn conflict() -> ApiError {
    ApiError {
        data: json!(null),
        status: Status::Conflict,
    }
}

pub fn unprocessable_entity() -> ApiError {
    ApiError {
        data: json!(null),
        status: Status::UnprocessableEntity,
    }
}

pub fn internal_server_error() -> ApiError {
    ApiError {
        data: json!(null),
        status: Status::InternalServerError,
    }
}

pub fn service_unavailable() -> ApiError {
    ApiError {
        data: json!(null),
        status: Status::ServiceUnavailable,
    }
}