use tide::{Body, Request, Response};

use crate::{error::err_server_error, state::State};

pub async fn get_user_statistics(req: Request<State>) -> tide::Result {
    let state = req.state();
    let client = &state.client;

    let mut context = client.new_context().unwrap();

    match client.get_user_statistics(&mut context) {
        Ok(user_stats) => Ok(Response::builder(201)
            .body(Body::from_json(&user_stats).unwrap())
            .build()),
        Err(_) => Ok(err_server_error()),
    }
}
