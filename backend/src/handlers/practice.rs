use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Result},
    Json,
};
use bson::oid::ObjectId;

use crate::{
    models::practice::Practice,
    requests::practice::{CreatePractice, RegisterOrUnregister},
    responses::practice::{Signup, UnregisterResponse, WaitlistNotification},
    routers::ApiState,
};

use super::ApiError;

pub(crate) async fn create_new_practice(
    State(api_state): State<ApiState>,
    Json(payload): Json<CreatePractice>,
) -> Result<impl IntoResponse, ApiError> {
    let practice = Practice::from(payload);

    api_state.practice_repo.create_practice(&practice).await?;

    Ok(StatusCode::CREATED)
}

pub(crate) async fn register(
    State(api_state): State<ApiState>,
    Json(payload): Json<RegisterOrUnregister>,
) -> Result<impl IntoResponse, ApiError> {
    let practice_id = ObjectId::parse_str(&payload.practice_id)?;

    let mut practice = api_state
        .practice_repo
        .get_practice_by_id(&practice_id)
        .await?
        .ok_or(ApiError {
            status: StatusCode::NOT_FOUND,
            message: format!("Practice with ID {} not found", practice_id),
        })?;

    let user = api_state
        .user_repo
        .get_user_by_discord_id(&payload.discord_id)
        .await?
        .ok_or(ApiError {
            status: StatusCode::NOT_FOUND,
            message: format!("User with Discord ID {} not found", payload.discord_id),
        })?;

    if practice.is_locked() {
        return Err(ApiError {
            status: StatusCode::BAD_REQUEST,
            message: "Practice not yet open".to_string(),
        });
    }

    match practice.register_user(&user) {
        Ok(on_waitlist) => {
            api_state
                .practice_repo
                .update_practice(&practice_id, &practice)
                .await?;

            let signup_response = Signup { on_waitlist };
            Ok((StatusCode::ACCEPTED, Json(signup_response)))
        }
        Err(e) => {
            return Err(ApiError {
                status: StatusCode::CONFLICT,
                message: e.to_string(),
            })
        }
    }
}

pub(crate) async fn unregister(
    State(api_state): State<ApiState>,
    Json(payload): Json<RegisterOrUnregister>,
) -> Result<impl IntoResponse, ApiError> {
    let practice_id = ObjectId::parse_str(&payload.practice_id)?;

    let mut practice = api_state
        .practice_repo
        .get_practice_by_id(&practice_id)
        .await?
        .ok_or(ApiError {
            status: StatusCode::NOT_FOUND,
            message: format!("Practice with ID {} not found", practice_id),
        })?;

    let user = api_state
        .user_repo
        .get_user_by_discord_id(&payload.discord_id)
        .await?
        .ok_or(ApiError {
            status: StatusCode::NOT_FOUND,
            message: format!("User with Discord ID {} not found", payload.discord_id),
        })?;

    if practice.is_locked() {
        return Err(ApiError {
            status: StatusCode::BAD_REQUEST,
            message: "Practice has been locked".to_string(),
        });
    }

    match practice.unregister_user(&user) {
        Ok(waitlistee) => {
            api_state
                .practice_repo
                .update_practice(&practice_id, &practice)
                .await?;

            match waitlistee {
                Some(user_id) => {
                    let waitlist_notification = WaitlistNotification {
                        practice_id: payload.practice_id.clone(),
                        discord_id: user_id.to_string(),
                    };
                    Ok((
                        StatusCode::ACCEPTED,
                        Json(UnregisterResponse::WaitlistNotification(
                            waitlist_notification,
                        )),
                    ))
                }
                None => Ok((StatusCode::ACCEPTED, Json(UnregisterResponse::Empty {}))),
            }
        }
        Err(e) => Err(ApiError {
            status: StatusCode::CONFLICT,
            message: e.to_string(),
        }),
    }
}
