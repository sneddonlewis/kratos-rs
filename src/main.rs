mod account_repo;
mod auth;
mod middleware;
mod view_models;

use crate::account_repo::{AccountRepoImpl, DynAccountRepo};
use crate::auth::{encode_token, get_public_jwk, Jwks};
use axum::extract::State;
use axum::http::{HeaderMap, HeaderValue, StatusCode};
use axum::middleware::from_extractor;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Extension, Json, Router};
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;

use crate::middleware::AuthorizationMiddleware;
use crate::view_models::{AccountAuthView, AccountDetailView};

#[tokio::main]
async fn main() {
    let account_repo = Arc::new(AccountRepoImpl) as DynAccountRepo;

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
        .with_state(account_repo);

    let listener = TcpListener::bind("localhost:3000").await.unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, router).await.unwrap();
}

fn serve_frontend() -> Router {
    Router::new().nest_service("/", ServeDir::new("web_client/dist/web_client/browser/"))
}

async fn account(
    Extension(claims): Extension<auth::Authorized>,
    State(account_repo): State<DynAccountRepo>,
) -> impl IntoResponse {
    let acc = account_repo.create().await.unwrap();
    let num = claims.0.card_num;
    if acc.card_number == num {
        Json(AccountDetailView::from(acc)).into_response()
    } else {
        (StatusCode::UNAUTHORIZED).into_response()
    }
}

async fn login(
    State(account_repo): State<DynAccountRepo>,
    Json(request): Json<AccountAuthView>,
) -> impl IntoResponse {
    let acc = account_repo
        .find(request.card_number.clone())
        .await
        .unwrap();

    if acc.pin == request.pin {
        let token = encode_token(request.card_number.clone());
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

async fn create_account(State(account_repo): State<DynAccountRepo>) -> impl IntoResponse {
    let acc = account_repo.create().await.unwrap();
    let view: AccountAuthView = AccountAuthView::from(acc);
    Json(view)
}
