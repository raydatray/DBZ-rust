/*
A tenant represents a "club" or any entity that
- Owns a distinct list of practices that no other tenant does
- Each user may only be associated to one tenant

We can configure various settings for a given club through the tenant struct such as
- The weekly release time of their practices
*/

use bson::oid::ObjectId;
use chrono::{NaiveTime, Weekday};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Tenant {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub(crate) id: Option<ObjectId>,
    name: String,
    practices: Vec<ObjectId>,
    members: Vec<ObjectId>,
    pub(crate) practice_release: WeeklyMoment,
}


/*
A moment in time on a given day that occurs every week

You must ensure that NaiveTime is in UTC before writing to it
*/
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct WeeklyMoment {
    pub(crate) day: Weekday,
    pub(crate) time: NaiveTime
}
