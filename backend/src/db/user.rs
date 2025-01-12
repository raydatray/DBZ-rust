use bson::{doc, oid::ObjectId};
use mongodb::Collection;
use tracing::debug;

use crate::models::user::User;

use super::DB;

#[derive(Clone)]
pub(crate) struct UserRepository {
    collection: Collection<User>,
}

impl UserRepository {
    pub(crate) fn new(db: &DB) -> Self {
        let collection = db.database.collection::<User>(DB::USER_COLLECTION);

        Self { collection }
    }

    pub(crate) async fn create_user(&self, user: &User) -> Result<(), anyhow::Error> {
        self.collection.insert_one(user).await?;

        Ok(())
    }

    async fn get_user_by_id(&self, user_id: &ObjectId) -> Result<Option<User>, anyhow::Error> {
        let user = self.collection.find_one(doc! {"_id": user_id}).await?;

        Ok(user)
    }

    pub(crate) async fn get_user_by_discord_id(
        &self,
        user_discord_id: &str,
    ) -> Result<Option<User>, anyhow::Error> {
        let user = self
            .collection
            .find_one(doc! {"discord_id": user_discord_id})
            .await?;

        Ok(user)
    }

    pub(crate) async fn get_user_by_email(
        &self,
        user_email: &str,
    ) -> Result<Option<User>, anyhow::Error> {
        let user = self
            .collection
            .find_one(doc! {"email" : user_email})
            .await?;

        Ok(user)
    }

    pub(crate) async fn set_user_discord_id(
        &self,
        user_id: &ObjectId,
        user_discord_id: &str,
    ) -> Result<(), anyhow::Error> {
        debug!(
            "setting discord id of user {}: {}",
            user_id, user_discord_id
        );

        self.collection
            .find_one_and_update(
                doc! {"_id" : user_id},
                doc! {"$set": {"discord_id" : user_discord_id}},
            )
            .await?;

        Ok(())
    }
}
