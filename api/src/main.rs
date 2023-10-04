use std::env;
use axum::{Json, Router};
use axum::extract::Query;
use axum::http::{HeaderValue, Method, StatusCode};
use axum::routing::get;
use lambda_http::{
    Error,
    run,
};
use serde::{Deserialize, Serialize};
use tower_http::cors::CorsLayer;

use sudoku_solver::Board;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let url = env::var("URL").expect("URL not set");
    let app = Router::new().route(
        "/",
        get(handler),
    )
        .route(
            "/sudoku",
            get(solve_sudoku),
        )
        .route("/random",
               get(get_random_sudoku)).layer(
        CorsLayer::new()
            .allow_origin(url.as_str().parse::<HeaderValue>().unwrap())
            .allow_methods(Method::GET)
    );
    //runs locally on :9000
    run(app)
        .await
}

#[derive(Serialize)]
struct JsonResponseSample {
    hello: String,
}

#[derive(Serialize, Deserialize)]
struct Sudoku {
    board: String,
}

async fn handler() -> Json<JsonResponseSample> {
    Json(JsonResponseSample {
        hello: String::from("World")
    })
}

async fn solve_sudoku(sudoku: Query<Sudoku>) -> Result<Json<Sudoku>, StatusCode> {
    let board = Board::new(sudoku.board.as_str())
        .map_err(|_| {
            StatusCode::FORBIDDEN
        })?;
    let solved = board.solve_board()
        .map_err(|_| {
            StatusCode::BAD_REQUEST
        })?;
    Ok(
        Json(
            Sudoku {
                board: solved.to_string()
            }
        )
    )
}


async fn get_random_sudoku() -> Json<Sudoku> {
    Json(
        Sudoku {
            board: String::from(
                "070004130000207006005013020001002000002190057003045802010378260367000580809001070"
            )
        }
    )
}

