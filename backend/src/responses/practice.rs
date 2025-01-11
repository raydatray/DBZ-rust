use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
#[serde(untagged)]
pub(crate) enum UnregisterResponse {
    WaitlistNotification(WaitlistNotification),
    Empty {},
}

#[derive(Serialize, ToSchema)]
pub(crate) struct Signup {
    pub(crate) on_waitlist: bool,
}

#[derive(Serialize, ToSchema)]
struct StartInfo {
    practice_id: String,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
    seat_count: usize,
    waitlist_count: usize,
}

#[derive(Serialize, ToSchema)]
pub(crate) struct WaitlistNotification {
    pub(crate) practice_id: String,
    pub(crate) discord_id: String,
}
