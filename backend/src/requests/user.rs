use serde::Deserialize;

use crate::models::user::{Gender, Side};

#[derive(Deserialize)]
pub(crate) struct AssociateDiscord {
    pub(crate) email: String,
    pub(crate) discord_id: String,
}

#[derive(Deserialize)]
pub(crate) struct NewUser {
    pub(crate) name: String,
    pub(crate) gender: Gender,
    pub(crate) email: String,
    pub(crate) side: Side,
}
