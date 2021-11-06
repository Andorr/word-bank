use lib::{PaginationOptions, WordQueryOptions};
use tide::{Body, Request, Response, convert::json};

use crate::state::State;

pub async fn words_list(req: Request<State>) -> tide::Result {
    let state = req.state();
    let client = &state.client;

    let mut res = Response::new(200);

    match client.query_words(WordQueryOptions::empty(), PaginationOptions::default()) {
        Ok(result) => {
            res.set_body(Body::from_json(&result)?);
        },
        Err(_) => {
            res.set_status(500);
            res.set_body(json!({
                "code": "SERVER_ERROR",
                "message": "internal server error"
            }));
        }
    };
    Ok(res)
}
