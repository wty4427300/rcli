use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use tracing::info;
use anyhow::Result;
use axum::Router;
use axum::routing::get;

#[derive(Debug)]
struct HttpServeState {
    path: PathBuf,
}

pub async fn process_http_server(path: &std::path::Path, port: u16) -> Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("serving {:?} on port {}", path, port);

    let router = Router::new().route("/", get(file_handler()));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}

async fn file_handler() -> &'static str {
    "Hello, World!"
}