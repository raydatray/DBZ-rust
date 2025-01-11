use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Result},
    Json,
};

use super::ApiError;
use crate::{
    models::user::User,
    requests::user::{AssociateDiscord, NewUser},
    routers::ApiState,
};

pub(crate) async fn create_new_user(
    State(api_state): State<ApiState>,
    Json(payload): Json<NewUser>,
) -> Result<impl IntoResponse, ApiError> {
    let existing_user = api_state
        .user_repo
        .get_user_by_email(&payload.email)
        .await?;

    if let Some(_) = existing_user {
        return Err(ApiError {
            status: StatusCode::CONFLICT,
            message: format!("User with email {} already exists", payload.email),
        });
    }

    let user = User::from(payload);

    api_state.user_repo.create_user(&user).await?;

    Ok(StatusCode::CREATED)
}

pub(crate) async fn associate_discord_to_user(
    State(api_state): State<ApiState>,
    Json(payload): Json<AssociateDiscord>,
) -> Result<impl IntoResponse, ApiError> {
    let user = api_state
        .user_repo
        .get_user_by_email(&payload.email)
        .await?
        .ok_or(ApiError {
            status: StatusCode::NOT_FOUND,
            message: format!("User with email {} not found", payload.email),
        })?;

    if let Some(_) = user.discord_id {
        return Err(ApiError {
            status: StatusCode::CONFLICT,
            message: format!(
                "User with email {} already has a Discord ID associated",
                payload.email
            ),
        });
    }

    api_state
        .user_repo
        .set_user_discord_id(&user.id.as_ref().unwrap(), &payload.discord_id)
        .await?;

    Ok(StatusCode::ACCEPTED)
}
