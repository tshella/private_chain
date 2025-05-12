use axum::{
    routing::{get, post},
    Router, Json, extract::State,
    serve,
};
use tokio::net::TcpListener;
use std::sync::{Arc, Mutex};
use crate::blockchain::Blockchain;
use crate::transaction::Transaction;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct TxRequest {
    sender: String,
    receiver: String,
    item: String,
}

pub async fn start_api(state: Arc<Mutex<Blockchain>>) {
    let app = Router::new()
        .route("/chain", get(get_chain))
        .route("/add", post(add_transaction))
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🚀 API server listening on {}", listener.local_addr().unwrap());

    serve(listener, app).await.unwrap();
}

async fn get_chain(State(state): State<Arc<Mutex<Blockchain>>>) -> Json<Vec<crate::block::Block>> {
    let chain = state.lock().unwrap().chain();
    Json(chain)
}

async fn add_transaction(
    State(state): State<Arc<Mutex<Blockchain>>>,
    Json(input): Json<TxRequest>,
) -> Json<crate::block::Block> {
    let tx = Transaction::new(input.sender, input.receiver, input.item);
    let block = state.lock().unwrap().add_block(vec![tx]);
    Json(block)
}
