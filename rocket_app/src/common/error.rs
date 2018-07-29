use std::io::Cursor;

use rocket::http::{ContentType, Status};
use rocket::request::Request;
use rocket::response::{Responder, Response};
use rocket_contrib::JsonValue;
use serde::Serialize;

#[derive(Debug)]
pub struct ApiError {
    data: JsonValue,
    status: Status,
}

impl ApiError {

    #[allow(dead_code)]
    /// Set the data of the `Response` to `data`.
    pub fn data<T: Serialize>(mut self, data: T) -> ApiError {
        self.data = json!(&data);
        self
    }

    #[allow(dead_code)]
    /// Convenience method to set `self.data` to `{"message": message}`.
    pub fn message(mut self, message: &str) -> ApiError {
        self.data = json!({ "message": message });
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

impl<T: Serialize + fina_util::error::Error + std::fmt::Debug> From<T> for ApiError {
    fn from(error: T) -> ApiError {
        let status = if error.is_internal_err() {
            error!("{:?}", error);
            Status::InternalServerError
        } else {
            Status::BadRequest
        };

        Self {
            status,
            data: json!(&error),
        }
    }
}

#[allow(dead_code)]
pub fn ok() -> ApiError {
    ApiError {
        data: json!(null),
        status: Status::Ok,
    }
}

#[allow(dead_code)]
pub fn created() -> ApiError {
    ApiError {
        data: json!(null),
        status: Status::Created,
    }
}

#[allow(dead_code)]
pub fn accepted() -> ApiError {
    ApiError {
        data: json!(null),
        status: Status::Accepted,
    }
}

#[allow(dead_code)]
pub fn no_content() -> ApiError {
    ApiError {
        data: json!(null),
        status: Status::NoContent,
    }
}

#[allow(dead_code)]
pub fn bad_request() -> ApiError {
    ApiError {
        data: json!(null),
        status: Status::BadRequest,
    }
}

#[allow(dead_code)]
pub fn unauthorized() -> ApiError {
    ApiError {
        data: json!(null),
        status: Status::Unauthorized,
    }
}

#[allow(dead_code)]
pub fn forbidden() -> ApiError {
    ApiError {
        data: json!(null),
        status: Status::Forbidden,
    }
}

#[allow(dead_code)]
pub fn not_found() -> ApiError {
    ApiError {
        data: json!(null),
        status: Status::NotFound,
    }
}

#[allow(dead_code)]
pub fn method_not_allowed() -> ApiError {
    ApiError {
        data: json!(null),
        status: Status::MethodNotAllowed,
    }
}

#[allow(dead_code)]
pub fn conflict() -> ApiError {
    ApiError {
        data: json!(null),
        status: Status::Conflict,
    }
}

#[allow(dead_code)]
pub fn unprocessable_entity() -> ApiError {
    ApiError {
        data: json!(null),
        status: Status::UnprocessableEntity,
    }
}

#[allow(dead_code)]
pub fn internal_server_error() -> ApiError {
    ApiError {
        data: json!(null),
        status: Status::InternalServerError,
    }
}

#[allow(dead_code)]
pub fn service_unavailable() -> ApiError {
    ApiError {
        data: json!(null),
        status: Status::ServiceUnavailable,
    }
}
