use axum::{
    extract::Query,
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone)]
struct AppState {
    allowed_roots: Vec<PathBuf>,
}

#[derive(Deserialize)]
struct BrowseParams {
    path: Option<String>,
}

#[derive(Serialize, Clone)]
struct FileEntry {
    name: String,
    path: String,
    is_dir: bool,
    size: u64,
    modified: Option<String>,
    extension: Option<String>,
}

#[derive(Serialize)]
struct BrowseResponse {
    current_path: String,
    parent_path: Option<String>,
    entries: Vec<FileEntry>,
}

#[derive(Serialize)]
struct RootsResponse {
    roots: Vec<String>,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

fn is_path_allowed(path: &Path, allowed_roots: &[PathBuf]) -> bool {
    allowed_roots.iter().any(|root| path.starts_with(root))
}

fn get_parent_path(path: &Path, allowed_roots: &[PathBuf]) -> Option<String> {
    let parent = path.parent()?;
    if is_path_allowed(parent, allowed_roots) {
        Some(parent.to_string_lossy().to_string())
    } else {
        None
    }
}

async fn browse(
    Query(params): Query<BrowseParams>,
    state: Arc<AppState>,
) -> Result<Json<BrowseResponse>, (StatusCode, Json<ErrorResponse>)> {
    let path = match &params.path {
        Some(p) => PathBuf::from(p),
        None => {
            return Ok(Json(BrowseResponse {
                current_path: "/".to_string(),
                parent_path: None,
                entries: state
                    .allowed_roots
                    .iter()
                    .map(|r| FileEntry {
                        name: r
                            .file_name()
                            .map(|n| n.to_string_lossy().to_string())
                            .unwrap_or_else(|| r.to_string_lossy().to_string()),
                        path: r.to_string_lossy().to_string(),
                        is_dir: true,
                        size: 0,
                        modified: None,
                        extension: None,
                    })
                    .collect(),
            }));
        }
    };

    let canonical = path.canonicalize().map_err(|_| {
        (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "Path not found".to_string(),
            }),
        )
    })?;

    if !is_path_allowed(&canonical, &state.allowed_roots) {
        return Err((
            StatusCode::FORBIDDEN,
            Json(ErrorResponse {
                error: "Access denied".to_string(),
            }),
        ));
    }

    if !canonical.is_dir() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Path is not a directory".to_string(),
            }),
        ));
    }

    let mut entries = Vec::new();
    let mut dir = tokio::fs::read_dir(&canonical).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: format!("Failed to read directory: {}", e),
            }),
        )
    })?;

    while let Some(entry) = dir.next_entry().await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: format!("Failed to read entry: {}", e),
            }),
        )
    })? {
        let metadata = entry.metadata().await.ok();
        let file_type = metadata.as_ref().map(|m| m.is_dir()).unwrap_or(false);
        let size = metadata.as_ref().map(|m| m.len()).unwrap_or(0);
        let modified = metadata
            .as_ref()
            .and_then(|m| m.modified().ok())
            .map(|t| {
                let datetime: chrono::DateTime<chrono::Utc> = t.into();
                datetime.format("%Y-%m-%d %H:%M:%S").to_string()
            });

        let name = entry.file_name().to_string_lossy().to_string();
        let extension = if !file_type {
            Path::new(&name)
                .extension()
                .map(|e| e.to_string_lossy().to_string())
        } else {
            None
        };

        entries.push(FileEntry {
            name,
            path: entry.path().to_string_lossy().to_string(),
            is_dir: file_type,
            size,
            modified,
            extension,
        });
    }

    entries.sort_by(|a, b| match (a.is_dir, b.is_dir) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
    });

    let parent_path = get_parent_path(&canonical, &state.allowed_roots);

    Ok(Json(BrowseResponse {
        current_path: canonical.to_string_lossy().to_string(),
        parent_path,
        entries,
    }))
}

async fn roots(state: Arc<AppState>) -> Json<RootsResponse> {
    Json(RootsResponse {
        roots: state
            .allowed_roots
            .iter()
            .map(|r| r.to_string_lossy().to_string())
            .collect(),
    })
}

#[tokio::main]
async fn main() {
    let allowed_roots: Vec<PathBuf> = std::env::args()
        .skip(1)
        .map(PathBuf::from)
        .filter(|p| p.exists() && p.is_dir())
        .collect();

    let allowed_roots = if allowed_roots.is_empty() {
        eprintln!("Usage: file-explorer-backend <dir1> [dir2] ...");
        eprintln!("No valid directories provided. Using current directory.");
        vec![std::env::current_dir().unwrap()]
    } else {
        allowed_roots
            .into_iter()
            .map(|p| p.canonicalize().unwrap())
            .collect()
    };

    println!("Allowed roots:");
    for root in &allowed_roots {
        println!("  - {}", root.display());
    }

    let state = Arc::new(AppState { allowed_roots });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/browse", get({
            let state = state.clone();
            move |query| browse(query, state)
        }))
        .route("/api/roots", get({
            let state = state.clone();
            move || roots(state)
        }))
        .layer(cors);

    let addr = "0.0.0.0:3001";
    println!("Server running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
