mod db;
mod error;
mod handlers;
mod jobs;
mod models;
mod requests;
mod responses;
mod routers;

use axum::{routing::get, Router};
use db::{practice::PracticeRepository, tenant::TenantRepository, user::UserRepository, DB};
use routers::ApiState;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt::init();

    let db = DB::init().await?;
    let practice_repo = PracticeRepository::new(&db);
    let user_repo = UserRepository::new(&db);
    let tenant_repo = TenantRepository::new(&db);

    let api_state = ApiState::new(practice_repo, user_repo, tenant_repo);

    let app = Router::new()
        .route("/", get(|| async { "hello world!" }))
        .nest("/practice", routers::practice::new(api_state.clone()))
        .nest("/user", routers::user::new(api_state.clone()));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    info!("server running on 0.0.0.0:3000");
    axum::serve(listener, app).await?;
    Ok(())
}
