use lib::quiz::QuizOptions;
use tide::{Body, Request, Response};

use crate::{
    error::{build_error_res, err_server_error},
    state::State,
};

pub async fn quiz_initialize(mut req: Request<State>) -> tide::Result {
    let quiz_options: QuizOptions = match req.body_json().await {
        Ok(options) => options,
        Err(err) => {
            return Ok(build_error_res(
                400,
                "INVALID_BODY",
                err.to_string().as_str(),
            ))
        }
    };

    let state = req.state();
    let client = &state.client;

    let mut ctx = client.new_context().unwrap();

    let quiz = match client.initialize_quiz(&mut ctx, quiz_options) {
        Ok(q) => q,
        Err(_) => return Ok(err_server_error()),
    };

    Ok(Response::builder(200).body(Body::from_json(&quiz)?).build())
}
