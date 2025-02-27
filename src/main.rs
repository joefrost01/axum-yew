pub mod api;
pub mod models;
pub mod utils;

use axum::{
    extract::Path,
    http::{header, HeaderValue, Method, StatusCode, Uri},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use rust_embed::RustEmbed;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tower_http::compression::{CompressionLayer, predicate::SizeAbove};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// Define the embedded frontend assets
#[derive(RustEmbed)]
#[folder = "frontend/dist/"]
struct FrontendAssets;

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "backend=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Log the list of embedded files for debugging
    tracing::info!("Listing embedded files:");
    for file in FrontendAssets::iter() {
        tracing::info!("- {}", file);

        // Print content of index.html if found
        if file == "index.html" {
            if let Some(content) = FrontendAssets::get(&file) {
                let content_str = std::str::from_utf8(&content.data).unwrap_or("Invalid UTF-8");
                tracing::info!("index.html content (first 100 chars): {}", &content_str[..std::cmp::min(100, content_str.len())]);
            }
        }
    }

    // CORS configuration
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any);

    tracing::info!("Starting server with embedded assets");

    // Configure compression - will compress responses larger than 1024 bytes
    let compression_layer = CompressionLayer::new()
        .compress_when(SizeAbove::new(1024));
        
    tracing::info!("Compression middleware enabled");

    // Build our application with routes
    let app = Router::new()
        .route("/health", get(|| async { "OK" }))
        .nest("/api", api::routes())
        .route("/", get(serve_index))  // Explicit route for index
        .route("/*file", get(serve_embedded_file))
        .layer(cors)
        .layer(compression_layer)  // Add compression before tracing so we can see it in the logs
        .layer(TraceLayer::new_for_http());

    // Run the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    tracing::info!("Server started, press Ctrl+C to stop");
    axum::serve(listener, app).await.unwrap();
}

// Explicit handler for index route
async fn serve_index() -> impl IntoResponse {
    tracing::info!("Serving index route");
    match FrontendAssets::get("index.html") {
        Some(content) => {
            tracing::info!("Found index.html");
            Response::builder()
                .header(header::CONTENT_TYPE, "text/html")
                .body(content.data.into())
                .unwrap()
        }
        None => {
            tracing::error!("index.html not found in embedded assets!");
            (StatusCode::NOT_FOUND, "index.html not found").into_response()
        }
    }
}

/// Serve static files from the embedded assets
async fn serve_embedded_file(
    uri: Uri,
    path: Path<String>,
) -> impl IntoResponse {
    let path = path.as_str();
    tracing::info!("Request for path: {}", path);

    let path = if path.is_empty() || path == "/" {
        "index.html"
    } else {
        // Remove the leading slash
        path.trim_start_matches('/')
    };

    tracing::info!("Looking for embedded file: {}", path);

    match FrontendAssets::get(path) {
        Some(content) => {
            tracing::info!("Found file: {}", path);
            let mime = mime_guess::from_path(path).first_or_octet_stream();
            tracing::info!("MIME type: {}", mime);
            Response::builder()
                .header(header::CONTENT_TYPE, mime.as_ref())
                .body(content.data.into())
                .unwrap()
        }
        None => {
            tracing::warn!("File not found: {}", path);
            if path != "index.html" {
                // Try to serve index.html for any non-existing path (SPA routing)
                tracing::info!("Trying to serve index.html instead");
                if let Some(content) = FrontendAssets::get("index.html") {
                    return Response::builder()
                        .header(header::CONTENT_TYPE, "text/html")
                        .body(content.data.into())
                        .unwrap();
                }
            }

            // If we reach here, the file wasn't found and we couldn't serve index.html either
            tracing::error!("No suitable file found to serve");
            (StatusCode::NOT_FOUND, "File not found").into_response()
        }
    }
}