// SPDX-FileCopyrightText: 2025 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use axum::{
    Router,
    http::StatusCode,
    routing::get,
};

fn create_router_v1() -> Router {
    Router::new().route("/health", get(|| async { StatusCode::OK }))
}

/// Function that abstracts the creation of the Router
/// for the API
pub fn create_router() -> Router {
    Router::new().nest("/v1", create_router_v1())
}
