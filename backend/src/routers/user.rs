use axum::{routing::post, Router};

use crate::handlers::user::{associate_discord_to_user, create_new_user};

use super::ApiState;

pub(crate) fn new(state: ApiState) -> Router {
    Router::new()
        .route("/", post(create_new_user))
        .route("/associate", post(associate_discord_to_user))
        .with_state(state)
}
