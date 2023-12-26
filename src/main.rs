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
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;

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
    let cors_middleware = CorsLayer::very_permissive();

    let router = Router::new()
        .layer(cors_middleware)
        .route("/account", get(account))
        .route_layer(from_extractor::<AuthorizationMiddleware>())
        .nest_service("/", ServeDir::new("web_client/dist/web_client/browser/"))
        .route("/new", get(create_account))
        .route("/login", post(login))
        .layer(Extension(jwks))
        .with_state(app_state);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, router).await.unwrap();
}

fn serve_frontend(router: Router) -> Router {
    Router::new().nest_service("/", ServeDir::new("web_client/dist/web_client/browser/"))
}

async fn account(
    Extension(claims): Extension<auth::Authorized>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let acc = state.account_repo.create().await.unwrap();
    let num = claims.0.card_num;
    if acc.card_number == num {
        Json(AccountDetailView::from(acc)).into_response()
    } else {
        (StatusCode::UNAUTHORIZED).into_response()
    }
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
