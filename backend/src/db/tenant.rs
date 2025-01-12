use bson::doc;
use mongodb::{Collection, Cursor};


use crate::models::tenant::Tenant;

use super::DB;

#[derive(Clone)]
pub(crate) struct TenantRepository{
    collection: Collection<Tenant>
}

impl TenantRepository {
    pub(crate) fn new(db: &DB) -> Self {
        let collection = db.database.collection::<Tenant>(DB::TENANT_COLLECTION);

        Self { collection }
    }

    pub(crate) async fn get_all(&self) -> Result<Cursor<Tenant>, anyhow::Error> {
        let tenant_cursor = self.collection.find(doc! {}).await?;
        Ok(tenant_cursor)
    }
}
