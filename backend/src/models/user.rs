use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::requests::user::NewUser;

#[derive(Debug, Serialize, Deserialize)]
enum UserType {
    Coach,
    Exec,
    Member,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum Gender {
    Female,
    Male,
    NA,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum Side {
    Left,
    NA,
    Right,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub(crate) id: Option<ObjectId>,
    name: String,
    gender: Gender,
    pub(crate) discord_id: Option<String>,
    email: String,
    pub(crate) side: Side,
}

impl From<NewUser> for User {
    fn from(new_user: NewUser) -> Self {
        Self {
            id: None,
            name: new_user.name,
            gender: new_user.gender,
            discord_id: None,
            email: new_user.email,
            side: new_user.side,
        }
    }
}
