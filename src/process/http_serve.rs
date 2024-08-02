use std::convert::Infallible;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use std::task::{Context, Poll};
use tracing::info;
use anyhow::Result;
use axum::body::Body;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use axum::Router;
use tower_http::services::ServeDir;
use tower_service::Service;
use futures::future::BoxFuture;

#[derive(Debug, Clone)]
struct HttpServeState {
    path: PathBuf,
}

// http://127.0.0.1:8081/
pub async fn process_http_server(path: PathBuf, port: u16) -> Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("serving {:?} on port {}", path, port);

    let state = HttpServeState { path: path.clone() };

    let dir_service = ServeDir::new(path)
        .append_index_html_on_directories(true)
        .precompressed_gzip()
        .precompressed_br()
        .precompressed_deflate()
        .precompressed_zstd()
        .fallback(ListDirService {
            state: Arc::new(state.clone()),
        });

    let router = Router::new()
        .nest_service("/", dir_service)
        .with_state(Arc::new(state));

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}

async fn list_dir(
    State(state): State<Arc<HttpServeState>>,
    path: &str,
) -> Result<Response, StatusCode> {
    let full_path = std::path::Path::new(&state.path).join(path);
    if !full_path.exists() {
        return Err(StatusCode::NOT_FOUND);
    }
    let mut html = String::from("<html><body><ul>");
    full_path.read_dir().unwrap().for_each(|entry| {
        if let Ok(entry) = entry {
            let path = entry.path();
            let name = path.file_name().unwrap().to_string_lossy();
            let mut dash = String::from("");
            if entry.path().is_dir() {
                dash = String::from("/");
            }
            html.push_str(&format!(
                "<li><a href=\"{}{}\">{}{}</a></li>",
                name, dash, name, dash
            ));
        }
    });
    html.push_str("</ul></body></html>");
    let response = Html(html).into_response();
    Ok(response)
}

#[derive(Debug, Clone)]
struct ListDirService {
    state: Arc<HttpServeState>,
}

impl<T> Service<axum::http::Request<T>> for ListDirService
where
    T: Send + 'static,
{
    type Response = Response;
    type Error = Infallible;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<std::result::Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: axum::http::Request<T>) -> Self::Future {
        let state = self.state.clone();
        let path = req.uri().path()[1..].to_string();

        Box::pin(async move {
            match list_dir(State(state), &path).await {
                Ok(response) => Ok(response),
                Err(status_code) => {
                    let response = Response::builder()
                        .status(status_code)
                        .body(Body::from(""))
                        .unwrap();
                    Ok(response)
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_file_handler() {
        let state = Arc::new(HttpServeState {
            path: PathBuf::from("."),
        });
        let (status, content) = file_handler(State(state), Path("Cargo.toml".to_string())).await;
        assert_eq!(status, StatusCode::OK);
        assert!(content.trim().starts_with("[package]"));
    }

    #[tokio::test]
    async fn test_file_handler1() {
        let path = PathBuf::from("./src");
        let state = Arc::new(HttpServeState { path: path.clone() });
        let result = list_dir(State(state), "process").await;
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }
}