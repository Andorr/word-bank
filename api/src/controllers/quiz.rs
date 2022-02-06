use lib::quiz::{QuizOptions, QuizQuestionResult, QuizResult};
use serde::{Deserialize, Serialize};
use tide::{Body, Request, Response};

use crate::{
    error::{build_error_res, err_server_error},
    state::State,
};

pub async fn initialize(mut req: Request<State>) -> tide::Result {
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

#[derive(Serialize, Deserialize)]
struct BodyQuizResultOptions {
    pub questions: Vec<QuizQuestionResult>,
}

impl From<BodyQuizResultOptions> for QuizResult {
    fn from(bqri: BodyQuizResultOptions) -> Self {
        QuizResult::new(bqri.questions)
    }
}

pub async fn insert_result(mut req: Request<State>) -> tide::Result {
    let quiz_result_options: BodyQuizResultOptions = match req.body_json().await {
        Ok(result) => result,
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

    let mut quiz_result: QuizResult = quiz_result_options.into();

    let mut context = client.new_context().unwrap();

    match client.insert_quiz_result(&mut context, &mut quiz_result) {
        Ok(_) => (),
        Err(_) => return Ok(err_server_error()),
    }

    match context.commit() {
        Ok(_) => Ok(Response::builder(201).build()),
        Err(_) => Ok(err_server_error()),
    }
}
