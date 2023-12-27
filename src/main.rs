mod auth;
mod middleware;
mod repo;
mod view_models;

use crate::auth::{encode_token, get_public_jwk, Jwks};
use axum::extract::State;
use axum::http::{HeaderMap, HeaderValue, StatusCode};
use axum::middleware::from_extractor;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Extension, Json, Router};
use repo::account_repo::{AccountRepoImpl, DynAccountRepo};
use repo::user_repo::{DynUserRepo, UserRepoImpl};
use std::sync::Arc;
use tokio::net::TcpListener;
// use tower_http::cors::CorsLayer;
use tower_http::services::{ServeDir, ServeFile};

use crate::middleware::AuthorizationMiddleware;
use crate::view_models::{AccountDetailView, User};

#[derive(Clone)]
struct AppState {
    pub account_repo: DynAccountRepo,
    pub user_repo: DynUserRepo,
}

#[tokio::main]
async fn main() {
    let app_state = AppState {
        account_repo: Arc::new(AccountRepoImpl) as DynAccountRepo,
        user_repo: Arc::new(UserRepoImpl) as DynUserRepo,
    };

    let jwks = Jwks(vec![get_public_jwk()]);

    // Define CORS middleware
    // let cors_middleware = CorsLayer::very_permissive();

    let frontend_assets_dir = "web_client/dist/web_client/browser/";
    let index_html = "web_client/dist/web_client/browser/index.html";
    let serve_dir =
        ServeDir::new(frontend_assets_dir).not_found_service(ServeFile::new(index_html));

    let router = Router::new()
        // .layer(cors_middleware)
        .route("/api/account", get(account))
        .route_layer(from_extractor::<AuthorizationMiddleware>())
        .route("/api/new", get(create_account))
        .route("/api/login", post(login))
        .layer(Extension(jwks))
        .with_state(app_state)
        .nest_service("/", serve_dir.clone())
        .fallback_service(serve_dir);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, router).await.unwrap();
}

async fn account(
    Extension(claims): Extension<auth::Authorized>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let acc = state.account_repo.create().await.unwrap();
    let name_claim = claims.0.username;
    println!("claim username: {:?}", name_claim);
    Json(AccountDetailView::from(acc)).into_response()
}

async fn login(State(state): State<AppState>, Json(request): Json<User>) -> impl IntoResponse {
    let user = state
        .user_repo
        .find(request.username.clone())
        .await
        .unwrap();

    if user.password == request.password {
        let token = encode_token(request.username.clone());
        let mut headers = HeaderMap::new();
        headers.insert(
            axum::http::header::AUTHORIZATION,
            HeaderValue::try_from(token).unwrap(),
        );
        (headers,).into_response()
    } else {
        (StatusCode::UNAUTHORIZED).into_response()
    }
}

async fn create_account(State(state): State<AppState>) -> impl IntoResponse {
    let acc = state.user_repo.create().await.unwrap();
    Json(acc)
}
