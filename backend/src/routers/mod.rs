pub(crate) mod practice;
pub(crate) mod user;

use crate::db::{practice::PracticeRepository, user::UserRepository};

#[derive(Clone)]
pub(crate) struct ApiState {
    pub(crate) practice_repo: PracticeRepository,
    pub(crate) user_repo: UserRepository,
}

impl ApiState {
    pub(crate) fn new(practice_repo: PracticeRepository, user_repo: UserRepository) -> Self {
        Self {
            practice_repo,
            user_repo,
        }
    }
}
