use std::fmt::Debug;

use lib::{PaginationOptions, Translation, Word, WordQueryOptions, WordUpdateOptions, WordType};
use serde::{Deserialize, Serialize};
use tide::{Request, Response, Body};
use uuid::Uuid;


use crate::{state::State, error::{err_server_error, build_error_res}};

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

    let res = match client.query_words(word_query_options, pagination_options.to_pagination_options()) {
        Ok(result) => Response::builder(200)
            .body(Body::from_json(&result)?)    
            .build(),
        Err(_) => err_server_error()
    };
    Ok(res)
}

#[derive(Serialize, Deserialize)]
struct BodyWordUpsertOptions {
    value: String,
    translations: Vec<String>,
    kind: WordType,
    tags: Option<Vec<String>>,
}

pub async fn word_create(mut req: Request<State>) -> tide::Result {

    let insert_options: BodyWordUpsertOptions = match req.body_json().await {
        Ok(options) => options,
        Err(err) => return Ok(build_error_res(400, "INVALID_BODY", err.to_string().as_str()))
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
        Ok(_) => Response::builder(201).build(),
        Err(_) => err_server_error()
    };

    Ok(response)
}

pub async fn word_update(mut req: Request<State>) -> tide::Result {
    let word_id = match Uuid::parse_str(req.param("id").unwrap()) {
        Ok(id) => id,
        Err(err) => return Ok(build_error_res(400, "INVALID_ID", err.to_string().as_str()))
    };
    
    let update_options: BodyWordUpsertOptions = match req.body_json().await {
        Ok(options) => options,
        Err(err) => return Ok(build_error_res(400, "INVALID_BODY", err.to_string().as_str()))
    };

    let state = req.state();
    let client = &state.client;

    let word_update_options = WordUpdateOptions{
        id: word_id,
        word: Some(update_options.value),
        kind: Some(update_options.kind),
        tags: update_options.tags,
        translations: Some(update_options.translations
            .iter()
            .map(|t| Translation::from_value(t))
            .collect()),
    };

    let response = match client.update_word(word_update_options) {
        Ok(_) => Response::builder(204).build(),
        Err(_) => err_server_error()
    };
    Ok(response)
}

pub async fn words_delete(req: Request<State>) -> tide::Result {
    let word_id = match Uuid::parse_str(req.param("id").unwrap()) {
        Ok(id) => id,
        Err(err) => return Ok(build_error_res(400, "INVALID_ID", err.to_string().as_str()))
    };

    let state = req.state();
    let client = &state.client;

    let res = match client.delete_word(word_id) {
        Ok(_) => Response::builder(204).build(),
        Err(_) => err_server_error()
    };
    Ok(res)
}