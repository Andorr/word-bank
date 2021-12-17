use std::convert::TryInto;
use std::fmt::{Debug};

use tide::{Response, convert::json, StatusCode};

pub fn err_server_error() -> Response {
    build_error_res(500, "SERVER_ERROR", "internal server error")
}

pub fn build_error_res<S>(status: S, code: &str, message: &str) -> Response where
    S: TryInto<StatusCode>,
    S::Error: Debug,
{
    Response::builder(status)
        .body(json!({
            "code": code,
            "message": message,
        }))
        .build()
}
