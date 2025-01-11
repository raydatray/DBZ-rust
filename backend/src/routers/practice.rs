use axum::{
    routing::{patch, post},
    Router,
};

use crate::handlers::practice::{create_new_practice, register, unregister};

use super::ApiState;

pub(crate) fn new(state: ApiState) -> Router {
    Router::new()
        .route("/", post(create_new_practice))
        .route("/register", patch(register))
        .route("/unregister", patch(unregister))
        .with_state(state)
}
