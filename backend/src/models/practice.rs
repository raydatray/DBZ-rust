use crate::requests::practice::CreatePractice;

use super::user::{Side, User};
use anyhow::anyhow;
use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Practice {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    date: DateTime<Utc>,
    duration: usize,
    left_side: Vec<Option<ObjectId>>,
    right_side: Vec<Option<ObjectId>>,
    left_waitlist: Vec<Option<ObjectId>>,
    right_waitlist: Vec<Option<ObjectId>>,
}

#[derive(Debug, Error)]
enum PracticeError {
    #[error("Practice and waitlist are full")]
    Full,
    #[error("User not found for this practice")]
    UserNotFound,
    #[error("Waitlist transfer failed")]
    WaitlistTransferFailed,
}

impl Practice {
    pub(crate) fn is_locked(&self) -> bool {
        let now = Utc::now();
        let unlock_time = self.date - chrono::Duration::hours(1);

        now < unlock_time
    }

    /// Registers the given user to the practice, automatically waitlisting them if necessary
    pub(crate) fn register_user(&mut self, user: &User) -> Result<bool, anyhow::Error> {
        let (spots, waitlist) = match user.side {
            Side::Left => (&mut self.left_side, &mut self.left_waitlist),
            Side::Right => (&mut self.right_side, &mut self.right_waitlist),
            _ => todo!("ambi??"),
        };

        if let Some(spot) = spots.iter_mut().find(|spot| spot.is_none()) {
            *spot = Some(user.id.unwrap());
            return Ok(false);
        }

        if let Some(spot) = waitlist.iter_mut().find(|spot| spot.is_none()) {
            *spot = Some(user.id.unwrap());
            return Ok(true);
        }

        Err(anyhow!(PracticeError::Full))
    }

    /// Unregisters the given user from the practice. If another user is unwaitlisted their Id is returned
    pub(crate) fn unregister_user(
        &mut self,
        user: &User,
    ) -> Result<Option<ObjectId>, anyhow::Error> {
        let (spots, waitlist) = match user.side {
            Side::Left => (&mut self.left_side, &mut self.left_waitlist),
            Side::Right => (&mut self.right_side, &mut self.right_waitlist),
            _ => todo!("ambi??"),
        };

        if let Some(pos) = spots
            .iter()
            .position(|id| id.as_ref() == Some(&user.id.as_ref().unwrap()))
        {
            spots[pos] = None;

            if let Some(waitlist_pos) = spots.iter().position(|id| id.is_some()) {
                let waitlist_user_id = waitlist[waitlist_pos].take();
                spots[pos] = waitlist_user_id;
                return Ok(waitlist_user_id);
            }

            return Ok(None);
        }
        Err(anyhow!(PracticeError::UserNotFound))
    }

    /// Transfers waitlistee's on prev to the current practice
    fn transfer_waitlist(&mut self, prev: &Practice) -> Result<(), anyhow::Error> {
        for waitlistee in prev.left_waitlist.iter().flatten() {
            if let Some(empty_spot) = self.left_side.iter_mut().find(|spot| spot.is_none()) {
                *empty_spot = Some(*waitlistee)
            } else {
                return Err(anyhow!(PracticeError::WaitlistTransferFailed));
            }
        }

        for waitlistee in prev.right_waitlist.iter().flatten() {
            if let Some(empty_spot) = self.right_side.iter_mut().find(|spot| spot.is_none()) {
                *empty_spot = Some(*waitlistee)
            } else {
                return Err(anyhow!(PracticeError::WaitlistTransferFailed));
            }
        }
        Ok(())
    }
}

impl From<CreatePractice> for Practice {
    fn from(create_practice: CreatePractice) -> Self {
        Self {
            id: None,
            date: create_practice.date,
            duration: create_practice.duration,
            left_side: vec![None; create_practice.side_count],
            right_side: vec![None; create_practice.side_count],
            left_waitlist: vec![None; create_practice.waitlist_count],
            right_waitlist: vec![None; create_practice.waitlist_count],
        }
    }
}
