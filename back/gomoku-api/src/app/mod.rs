use crate::{
    config::app_state::ArcAppState,
    controller::{openapi::openapi_route, room::room_router, test::test_router},
};
use axum::{
    http::{header, Method},
    Router,
};
use tower_http::{compression::CompressionLayer, cors::CorsLayer, limit::RequestBodyLimitLayer};

pub async fn create_app() -> Router {
    let arc_app_state = ArcAppState::new().await;
    Router::new()
        .merge(openapi_route(arc_app_state.clone()))
        .merge(room_router(arc_app_state.clone()))
        .merge(test_router(arc_app_state.clone()))
        .with_state(arc_app_state)
        .layer(CompressionLayer::new())
        .layer(RequestBodyLimitLayer::new(1024 * 1024))
        .layer(
            CorsLayer::new()
                .allow_origin([
                    "http://localhost:5174".parse().unwrap(),
                    "http://localhost:4173".parse().unwrap(),
                    "https://lsy969999.github.io".parse().unwrap(),
                ])
                .allow_credentials(true)
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::OPTIONS,
                    Method::PUT,
                    Method::DELETE,
                    Method::PATCH,
                ])
                .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION]),
        )
}
