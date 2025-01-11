use chrono::{DateTime, Utc};
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub(crate) struct CreatePractice {
    pub(crate) date: DateTime<Utc>,
    pub(crate) duration: usize,
    pub(crate) side_count: usize,
    pub(crate) waitlist_count: usize,
}

#[derive(Deserialize, ToSchema)]
pub(crate) struct RegisterOrUnregister {
    pub(crate) practice_id: String,
    pub(crate) discord_id: String,
}
