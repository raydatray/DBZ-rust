use std::sync::Arc;

use bson::oid::ObjectId;
use chrono::Timelike;
use tokio_cron_scheduler::{Job, JobScheduler};
use tracing::{error,info};

use crate::routers::ApiState;

pub(crate) async fn start_scheduler(api_state: ApiState) -> Result<(), anyhow::Error> {
    let api_state = Arc::new(api_state);
    let mut tenants = api_state.tenant_repo.get_all().await?;

    let scheduler = JobScheduler::new().await?;

    while tenants.advance().await? {
        let tenant = tenants.deserialize_current()?;
        let practice_release_time = tenant.practice_release;

        let api_state_clone_for_transfer = Arc::clone(&api_state);
        let api_state_clone_for_release = Arc::clone(&api_state);

        let transfer_job = Job::new_async(
            format!(
                "0 {} {} ** {}",
                practice_release_time.time.hour(),
                practice_release_time.time.minute(),
                practice_release_time.day.num_days_from_monday()
            ),
            move |_uuid, _lock| {
                let api_state = Arc::clone(&api_state_clone_for_transfer);
                Box::pin(
                    async move {
                        match transfer_waitlist(api_state.clone()).await {
                            Ok(waitlistees) => {
                                if let Some(waitlistees) = waitlistees {
                                    if let Err(e) = notify_waitlistees(&waitlistees).await {
                                        error!("Failed to notify waitlistees, erorr: {}", e);
                                    }
                                }
                            },
                            Err(e) => {
                                error!("Failed to transfer for tenant: {:?}, error: {}", tenant.id, e);
                            }
                        }
                    }
                )
            }
        )?;
        scheduler.add(transfer_job).await?;

        let release_job = Job::new_async(
            format!(
                "0 {} {} ** {}",
                practice_release_time.time.hour(),
                practice_release_time.time.minute(),
                practice_release_time.day.num_days_from_monday()
            ),
            move |_uuid, _lock| {
                let api_state = Arc::clone(&api_state_clone_for_release);
                Box::pin(
                    async move {
                        if let Err(e) = notify_practice_open(api_state.clone()).await {
                            error!("Failedto notify newly opened practice, error: {}", e);
                        }
                    }
                )
            }
        )?;
        scheduler.add(release_job).await?;

    }

    Ok(())
}

async fn transfer_waitlist(api_state: Arc<ApiState>) -> Result<Option<Vec<ObjectId>>, anyhow::Error> {
    let mut practices = api_state.practice_repo.get_within_next_week().await?;
    let mut transfered_users = None;

    while practices.advance().await? {
        let mut practice = practices.deserialize_current()?;

        match api_state.practice_repo.get_previous(&practice).await? {
            Some(previous_practice) => {
                transfered_users = Some(practice.transfer_waitlist(&previous_practice)?);

                api_state
                    .practice_repo
                    .update_practice(practice.id.as_ref().unwrap(), &practice)
                    .await?;
            }
            None => info!("Failed to find a previous practice for {:?}", practice),
        }
    }
    Ok(transfered_users)
}

async fn notify_waitlistees(waitlistees: &Vec<ObjectId>) -> Result<(), anyhow::Error> {
    waitlistees.iter().for_each(|user_id| println!("Notifying user {}", user_id));
    Ok(())
}


async fn notify_practice_open(api_state: Arc<ApiState>) -> Result<(), anyhow::Error> {
    println!("Notifying that practice is open");
    Ok(())
}
