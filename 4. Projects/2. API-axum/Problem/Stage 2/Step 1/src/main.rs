// TODO: import log, pretty_env_logger, dotenv, and PgPoolOptions
extern crate pretty_env_logger;
extern crate dotenv;
#[macro_use] extern crate log;
use std::net::SocketAddr;
use std::env;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;

use axum::{
    routing::{delete, get, post},
    Router,
};

mod handlers;
mod models;

use handlers::*;

#[tokio::main]
async fn main() {
    // TODO: Initialize pretty_env_logger
    pretty_env_logger::init();
    // TODO: Initialize dotenv
    dotenv::dotenv();

    // Create a new PgPoolOptions instance with a maximum of 5 connections.
    // Use dotenv to get the database url.
    // Use the `unwrap` or `expect` method instead of handling errors. If an
    // error occurs at this stage the server should be terminated.
    // See examples on GitHub page: https://github.com/launchbadge/sqlx
    let pool = PgPoolOptions::new().max_connections(5).connect(&env::var("DATABASE_URL").unwrap()).await.unwrap();

    // Using slqx, execute a SQL query that selects all questions from the questions table.
    // Use the `unwrap` or `expect` method to handle errors. This is just some test code to
    // make sure we can connect to the database.
    let recs = sqlx::query!("SELECT * FROM questions").fetch_all(&pool).await.unwrap();

    info!("********* Question Records *********");
    // TODO: Log recs with debug formatting using the info! macro
    info!("The following values are contained in the quesiton database:\n
    {:?}", recs);

    let app = Router::new()
        .route("/question", post(create_question))
        .route("/questions", get(read_questions))
        .route("/question", delete(delete_question))
        .route("/answer", post(create_answer))
        .route("/answers", get(read_answers))
        .route("/answer", delete(delete_answer));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
