use bson::{doc, oid::ObjectId};
use chrono::{Duration, Utc};
use mongodb::{Collection, Cursor};

use crate::models::practice::Practice;

use super::DB;

#[derive(Clone)]
pub(crate) struct PracticeRepository {
    collection: Collection<Practice>,
}

impl PracticeRepository {
    pub(crate) fn new(db: &DB) -> Self {
        let collection = db.database.collection::<Practice>(DB::PRACTICE_COLLECITON);
        Self { collection }
    }

    pub(crate) async fn create_practice(&self, practice: &Practice) -> Result<(), anyhow::Error> {
        self.collection.insert_one(practice).await?;
        Ok(())
    }

    pub(crate) async fn get_practice_by_id(
        &self,
        practice_id: &ObjectId,
    ) -> Result<Option<Practice>, anyhow::Error> {
        let practice = self.collection.find_one(doc! {"_id" : practice_id}).await?;
        Ok(practice)
    }

    pub(crate) async fn get_within_next_week(&self) -> Result<Cursor<Practice>, anyhow::Error> {
        let now = Utc::now();
        let next_week = now + Duration::weeks(1);

        let practice_cursor = self
            .collection
            .find(doc! {
                "date" : {
                    "$gte": now,
                    "$lt" : next_week
                }
            })
            .await?;

        Ok(practice_cursor)
    }

    pub(crate) async fn get_previous(
        &self,
        curr_practice: &Practice,
    ) -> Result<Option<Practice>, anyhow::Error> {
        let curr_time = curr_practice.date;
        let prev_time = curr_time - Duration::weeks(1);

        let prev_practice = self
            .collection
            .find_one(doc! {
                "date" : prev_time
            })
            .await?;

        Ok(prev_practice)
    }

    async fn get_all(&self) -> Result<Cursor<Practice>, anyhow::Error> {
        let practice_cursor = self.collection.find(doc! {}).await?;
        Ok(practice_cursor)
    }

    pub(crate) async fn update_practice(
        &self,
        practice_id: &ObjectId,
        updated_practice: &Practice,
    ) -> Result<(), anyhow::Error> {
        self.collection
            .replace_one(doc! {"_id": practice_id}, updated_practice)
            .await?;
        Ok(())
    }
}
