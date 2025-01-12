pub(crate) mod practice;
pub(crate) mod user;

use crate::db::{practice::PracticeRepository, tenant::TenantRepository, user::UserRepository};

#[derive(Clone)]
pub(crate) struct ApiState {
    pub(crate) practice_repo: PracticeRepository,
    pub(crate) user_repo: UserRepository,
    pub(crate) tenant_repo: TenantRepository,
}

impl ApiState {
    pub(crate) fn new(
        practice_repo: PracticeRepository,
        user_repo: UserRepository,
        tenant_repo: TenantRepository,
    ) -> Self {
        Self {
            practice_repo,
            user_repo,
            tenant_repo,
        }
    }
}
