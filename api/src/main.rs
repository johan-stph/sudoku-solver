use axum::{Json, Router};
use axum::extract::Query;
use axum::http::StatusCode;
use axum::routing::get;
use lambda_http::{
    run,
    Error,
};
use serde::Serialize;
use sudoku_solver::{Board, Field};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let app = Router::new().route(
        "/",
        get(handler)
    )
        .route(
            "/sudoku",
            get(solve_sudoku)
        );
    run(app)
        .await
}

#[derive(Serialize)]
struct JsonResponseSample {
    hello: String
}

#[derive(Serialize)]
struct Sudoku {
    board: String
}

async fn handler() -> Json<JsonResponseSample> {
    Json(JsonResponseSample {
        hello: String::from("World")
    })
}

async fn solve_sudoku(sudoku: Query<Sudoku>) -> Result<Json(Sudoku), StatusCode> {
    let board = Board::new(sudoku.board.as_str())?;
    let solved = board.solve_board()
    Json(Sudoku {
        
    })
}

