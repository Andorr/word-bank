use lib::{
    FolderUpdateOptions, PaginationOptions, Translation, Word, WordQueryOptions, WordType,
    WordUpdateOptions,
};
use serde::{Deserialize, Serialize};
use tide::{Body, Request, Response};
use uuid::Uuid;

use crate::{
    constants::ROOT_FOLDER,
    error::{build_error_res, err_server_error},
    models::PaginationOptionalOptions,
    state::State,
};

impl PaginationOptionalOptions {
    pub fn to_pagination_options(&self) -> PaginationOptions {
        PaginationOptions::new(self.limit.unwrap_or(25), self.page.unwrap_or(1))
    }
}

pub async fn list(req: Request<State>) -> tide::Result {
    let state = req.state();
    let client = &state.client;

    let word_query_options = req.query::<WordQueryOptions>()?;
    let pagination_options = req.query::<PaginationOptionalOptions>()?;

    let res = match client.query_words(
        word_query_options,
        pagination_options.to_pagination_options(),
    ) {
        Ok(result) => Response::builder(200)
            .body(Body::from_json(&result)?)
            .build(),
        Err(_) => err_server_error(),
    };

    Ok(res)
}

#[derive(Serialize, Deserialize)]
struct BodyWordUpsertOptions {
    value: String,
    translations: Vec<String>,
    kind: WordType,
    tags: Option<Vec<String>>,
    folder: Option<Uuid>,
}

pub async fn create(mut req: Request<State>) -> tide::Result {
    let insert_options: BodyWordUpsertOptions = match req.body_json().await {
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

    // TODO: Make a ::from_options function for this
    let mut word = Word::from_value(&insert_options.value);
    word.kind = insert_options.kind;
    word.translations = insert_options
        .translations
        .iter()
        .map(|t| Translation::from_value(t))
        .collect();
    if let Some(tags) = insert_options.tags {
        word.tags = tags;
    }

    let mut context = client.new_context().unwrap();

    // Insert word
    let word_result = client.insert_word(&mut context, &mut word);
    if word_result.is_err() {
        return Ok(err_server_error());
    }

    // Add word into folder
    let folder_result = client.update_folder(
        &mut context,
        FolderUpdateOptions {
            id: insert_options
                .folder
                .unwrap_or(Uuid::parse_str(ROOT_FOLDER).unwrap()),
            name: None,
            parent: None,
            add: Some(vec![word.id]),
            remove: None,
        },
    );
    if folder_result.is_err() {
        return Ok(err_server_error());
    }
    // Commit
    let response = match context.commit() {
        Ok(_) => Response::builder(201).body(Body::from_json(&word)?).build(),
        Err(_) => err_server_error(),
    };
    Ok(response)
}

pub async fn update(mut req: Request<State>) -> tide::Result {
    let word_id = match Uuid::parse_str(req.param("id").unwrap()) {
        Ok(id) => id,
        Err(err) => return Ok(build_error_res(400, "INVALID_ID", err.to_string().as_str())),
    };

    let update_options: BodyWordUpsertOptions = match req.body_json().await {
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

    let word_update_options = WordUpdateOptions {
        id: word_id,
        word: Some(update_options.value),
        kind: Some(update_options.kind),
        tags: update_options.tags,
        translations: Some(
            update_options
                .translations
                .iter()
                .map(|t| Translation::from_value(t))
                .collect(),
        ),
    };

    let response = match client.update_word(word_update_options) {
        Ok(_) => Response::builder(204).build(),
        Err(_) => err_server_error(),
    };
    Ok(response)
}

pub async fn delete(req: Request<State>) -> tide::Result {
    let word_id = match Uuid::parse_str(req.param("id").unwrap()) {
        Ok(id) => id,
        Err(err) => return Ok(build_error_res(400, "INVALID_ID", err.to_string().as_str())),
    };

    let state = req.state();
    let client = &state.client;

    let mut ctx = client.new_context().unwrap();

    // TODO: Add context, and delete word from corresponding folders!
    let res = match client.delete_word(&mut ctx, word_id) {
        Ok(_) => Response::builder(204).build(),
        Err(_) => err_server_error(),
    };

    match ctx.commit() {
        Ok(_) => Ok(res),
        Err(_) => Ok(err_server_error()),
    }
}

#[derive(Deserialize)]
struct WordRandomQueryParams {
    count: Option<u32>,
}

pub async fn random(req: Request<State>) -> tide::Result {
    let query_params: WordRandomQueryParams = match req.query() {
        Ok(params) => params,
        Err(err) => {
            return Ok(build_error_res(
                400,
                "INVALID_QUERY_PARAMS",
                err.to_string().as_str(),
            ))
        }
    };

    let state = req.state();
    let client = &state.client;

    let mut ctx = client.new_context().unwrap();

    let res = match client.get_random_words(&mut ctx, query_params.count.unwrap_or(16)) {
        Ok(words) => Response::builder(200)
            .body(Body::from_json(&words)?)
            .build(),
        Err(_) => err_server_error(),
    };

    match ctx.commit() {
        Ok(_) => Ok(res),
        Err(_) => Ok(err_server_error()),
    }
}
