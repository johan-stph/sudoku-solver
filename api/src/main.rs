use std::net::SocketAddr;
use axum::{Json, Router};
use axum::routing::get;
use serde::Serialize;
use sudoku_solver::Field;

#[tokio::main]
async fn main() {
    let app = Router::new().route(
        "/",
        get(handler)
    )
        .route(
            "/sudoku",
            get(solve_sudoku)
        );
    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Serialize)]
struct JsonResponseSample {
    hello: String
}

#[derive(Serialize)]
struct SudokuResult {
    result: Vec<Field>
}

async fn handler() -> Json<JsonResponseSample> {
    Json(JsonResponseSample {
        hello: String::from("World")
    })
}

async fn solve_sudoku() -> Json<SudokuResult> {
    Json(SudokuResult {
        result: vec![]
    })
}

