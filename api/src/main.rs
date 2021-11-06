mod controllers;
mod middleware;
mod state;

use std::env;
use lib::utils::load_env;
use tide::{security::{CorsMiddleware, Origin}};

use crate::{middleware::auth::Authorization, state::State};



#[async_std::main]
async fn main() -> tide::Result<()> {

    let _ = load_env();

    let port = match env::var("PORT") {
        Ok(port) => port,
        Err(_) => "8080".to_string()
    };

    tide::log::start();
    let mut app = tide::with_state(State::new());

    app.with(
        CorsMiddleware::new()
        .allow_origin(Origin::from("*"))
    );

    let authorization = Authorization::new(
        env::var("WORDBANK_AUTH_TOKEN")
        .expect("'WORDBANK_AUTH_TOKEN' env expected")
    );

    app
        .at("/api/v1/words")
        .with(authorization)
        .get(controllers::words_list);


    app.listen(format!("0.0.0.0:{}", port)).await?;

    Ok(())
}