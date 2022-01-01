use std::fmt::Debug;

use lib::{Folder};
use serde::{Deserialize, Serialize};
use tide::{Request, Response, Body};
use uuid::Uuid;

use crate::{state::State, error::{build_error_res, err_server_error}};


#[derive(Debug, Clone, Deserialize, Serialize)]
struct BodyFolderUpsertOptions {
    id: Option<Uuid>,
    name: String,
    parent: Option<Uuid>,
}

pub async fn folder_create(mut req: Request<State>) -> tide::Result {
    let insert_options: BodyFolderUpsertOptions = match req.body_json().await {
        Ok(options) => options,
        Err(err) => return Ok(build_error_res(400, "INVALID_BODY", err.to_string().as_str()))
    };

    let mut folder = Folder::new(insert_options.name.as_str());
    if let Some(id) = insert_options.id {
        folder.id = id;
    }
    folder.parent = insert_options.parent;

    let state = req.state();
    let client = &state.client;

    let mut context = client.new_context().unwrap();
    

    if let Some(parent) = folder.parent {
        match client.get_folder(parent) {
            Ok(_) => (),
            Err(_) => return Ok(build_error_res(400, "FOLDER_NOT_EXISTS", format!("parent folder by id '{}' does not exists", parent.to_string()).as_str()))
        }
    }

    match client.insert_folder(&mut context, &mut folder) {
        Ok(_) => (),
        Err(_) => return Ok(err_server_error())
    }

    let response = match context.commit() {
        Ok(_) => Response::builder(201)
        .body(Body::from_json(&folder)?)
        .build(),
        Err(_) => err_server_error(),
    };

    Ok(response)
}