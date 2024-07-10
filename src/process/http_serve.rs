use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tracing::{info, warn};
use anyhow::Result;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Router;
use axum::routing::get;
use tower_http::services::ServeDir;

#[derive(Debug)]
struct HttpServeState {
    path: PathBuf,
}

pub async fn process_http_server(path: PathBuf, port: u16) -> Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("serving {:?} on port {}", path, port);

    let state = HttpServeState { path: path.clone() };
    let dir_service = ServeDir::new(path)
        .append_index_html_on_directories(true)
        .precompressed_gzip()
        .precompressed_br()
        .precompressed_deflate();
    let router = Router::new()
        .route("/*path", get(file_handler))
        .nest_service("/tower", dir_service)
        .with_state(Arc::new(state));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}

async fn file_handler(State(_state): State<Arc<HttpServeState>>, Path(_path): Path<String>) -> (StatusCode, String) {
    let p = std::path::Path::new(&_state.path).join(_path);
    if !p.exists() {
        return (StatusCode::NOT_FOUND, format!("file {} not Found", p.display()));
    } else {
        match tokio::fs::read_to_string(p).await {
            Ok(content) => {
                info!("read {} bytes",content.len());
                (StatusCode::OK, content)
            }
            Err(e) => {
                warn!("read file error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            }
        }
    }
}