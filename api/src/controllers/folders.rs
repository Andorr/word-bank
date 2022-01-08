use std::fmt::Debug;

use lib::{Folder, FolderContent, FolderQueryOptions, FolderUpdateOptions};
use serde::{Deserialize, Serialize};
use tide::{Body, Request, Response};
use uuid::Uuid;

use crate::{
    error::{build_error_res, err_server_error},
    models::PaginationOptionalOptions,
    state::State,
};

pub async fn folders_list(req: Request<State>) -> tide::Result {
    let state = req.state();
    let client = &state.client;

    let folder_query_options = req.query::<FolderQueryOptions>()?;
    let pagination_options = req.query::<PaginationOptionalOptions>()?;

    let mut ctx = client.new_context().unwrap();
    let res = match client.query_folders(
        &mut ctx,
        folder_query_options,
        pagination_options.to_pagination_options(),
    ) {
        Ok(result) => Response::builder(200)
            .body(Body::from_json(&result)?)
            .build(),
        Err(_) => err_server_error(),
    };
    let _ = ctx.commit();

    Ok(res)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct FolderResult {
    data: Folder,
    content: FolderContent,
}

pub async fn folder_get(req: Request<State>) -> tide::Result {
    let folder_id = match Uuid::parse_str(req.param("id").unwrap()) {
        Ok(id) => id,
        Err(err) => return Ok(build_error_res(400, "INVALID_ID", err.to_string().as_str())),
    };

    let state = req.state();
    let client = &state.client;

    let mut context = client.new_context().unwrap();

    let folder = match client.get_folder(&mut context, folder_id) {
        Ok(f) => f,
        Err(_) => return Ok(err_server_error()),
    };

    let content = match client.get_folder_content(&mut context, &folder) {
        Ok(c) => c,
        Err(_) => return Ok(err_server_error()),
    };

    let response = Response::builder(200)
        .body(Body::from_json(&FolderResult {
            data: folder,
            content: content,
        })?)
        .build();

    Ok(response)
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct BodyFolderUpsertOptions {
    id: Option<Uuid>,
    name: Option<String>,
    parent: Option<Option<Uuid>>,
}

pub async fn folder_create(mut req: Request<State>) -> tide::Result {
    let insert_options: BodyFolderUpsertOptions = match req.body_json().await {
        Ok(options) => options,
        Err(err) => {
            return Ok(build_error_res(
                400,
                "INVALID_BODY",
                err.to_string().as_str(),
            ))
        }
    };
    if insert_options.name.is_none() {
        return Ok(build_error_res(
            400,
            "INVALID_BODY",
            "missing required property 'name'",
        ));
    }

    let mut folder = Folder::new(insert_options.name.unwrap().as_str());
    if let Some(id) = insert_options.id {
        folder.id = id;
    }
    if let Some(parent) = insert_options.parent {
        folder.parent = parent;
    }

    let state = req.state();
    let client = &state.client;

    let mut context = client.new_context().unwrap();

    if let Some(parent) = folder.parent {
        match client.get_folder(&mut context, parent) {
            Ok(_) => (),
            Err(_) => {
                return Ok(build_error_res(
                    400,
                    "FOLDER_NOT_EXISTS",
                    format!(
                        "parent folder by id '{}' does not exists",
                        parent.to_string()
                    )
                    .as_str(),
                ))
            }
        }
    }

    match client.insert_folder(&mut context, &mut folder) {
        Ok(_) => (),
        Err(_) => return Ok(err_server_error()),
    }

    let response = match context.commit() {
        Ok(_) => Response::builder(201)
            .body(Body::from_json(&folder)?)
            .build(),
        Err(_) => err_server_error(),
    };

    Ok(response)
}

pub async fn folder_update(mut req: Request<State>) -> tide::Result {
    let folder_id = match Uuid::parse_str(req.param("id").unwrap()) {
        Ok(id) => id,
        Err(err) => return Ok(build_error_res(400, "INVALID_ID", err.to_string().as_str())),
    };

    let update_options: BodyFolderUpsertOptions = match req.body_json().await {
        Ok(options) => options,
        Err(err) => {
            return Ok(build_error_res(
                400,
                "INVALID_BODY",
                err.to_string().as_str(),
            ))
        }
    };

    let folder_update_options = FolderUpdateOptions {
        id: folder_id,
        name: update_options.name,
        parent: update_options.parent,
        add: None,
        remove: None,
    };

    let state = req.state();
    let client = &state.client;

    let mut context = client.new_context().unwrap();

    let response = match client.update_folder(&mut context, folder_update_options) {
        Ok(_) => Response::builder(204).build(),
        Err(_) => err_server_error(),
    };

    let _ = context.commit();
    Ok(response)
}

pub async fn folder_delete(req: Request<State>) -> tide::Result {
    let folder_id = match Uuid::parse_str(req.param("id").unwrap()) {
        Ok(id) => id,
        Err(err) => return Ok(build_error_res(400, "INVALID_ID", err.to_string().as_str())),
    };

    let state = req.state();
    let client = &state.client;

    let mut context = client.new_context().unwrap();

    let response = match client.delete_folder(&mut context, folder_id) {
        Ok(_) => Response::builder(204).build(),
        Err(_) => err_server_error(),
    };

    let _ = context.commit();
    Ok(response)
}
