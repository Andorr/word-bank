use lib::{PaginationOptions, Translation, Word, WordQueryOptions, WordType};
use serde::{Deserialize, Serialize};
use tide::{Body, Request, Response, convert::json};

use crate::state::State;

#[derive(Debug, Clone, Deserialize)]
struct PaginationOptionalOptions {
    limit: Option<usize>,
    page: Option<usize>,
}

impl PaginationOptionalOptions {
    pub fn to_pagination_options(&self) -> PaginationOptions {
        PaginationOptions::new(
            self.limit.unwrap_or(25),
            self.page.unwrap_or(1),
        )
    }
}

pub async fn words_list(req: Request<State>) -> tide::Result {
    let state = req.state();
    let client = &state.client;

    let word_query_options = req.query::<WordQueryOptions>()?;
    let pagination_options = req.query::<PaginationOptionalOptions>()?;

    println!("{:?}", word_query_options);
    println!("{:?}", pagination_options);

    let mut res = Response::new(200);

    match client.query_words(word_query_options, pagination_options.to_pagination_options()) {
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

#[derive(Serialize, Deserialize)]
struct WordInsertOptions {
    value: String,
    translations: Vec<String>,
    kind: WordType,
    tags: Option<Vec<String>>,
}

pub async fn word_create(mut req: Request<State>) -> tide::Result {

    let insert_options: WordInsertOptions = match req.body_json().await {
        Ok(options) => options,
        Err(err) => {
            let response = Response::builder(400)
                .body(json!({
                    "code": "INVALID_BODY",
                    "message": err.to_string(),
                }))
                .build();
            return Ok(response)
        }
    };

    let state = req.state();
    let client = &state.client;

    let mut word = Word::from_value(&insert_options.value);
    word.kind = insert_options.kind;
    word.translations = insert_options.translations
        .iter()
        .map(|t| Translation::from_value(t))
        .collect();
    if let Some(tags) = insert_options.tags {
        word.tags = tags;
    }

    let response = match client.insert_word(&mut word) {
        Ok(_) => 
            Response::builder(201)
                .body(Body::from_json(&word)?)
                .build()
        ,
        Err(_) => {
            Response::builder(500)
                .body(json!({
                    "code": "SERVER_ERROR",
                    "message": "internal server error"
                }))
                .build()
        }
    };

    Ok(response)
}