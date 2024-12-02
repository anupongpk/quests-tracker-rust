use quests_tracker::{
    config::config_loader, infrastructure::postgres::postgress_connection::establish_connection,
}
use tracing::{error, info};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let dotenvy_env = match config_loader::load() {
        Ok(env) => env,
        Err(err) => {
            error!("Failed to load ENV {}", err);
            std::process::exit(1);
        }
    };

    info!("ENV has been loaded");

    let postgres_pool = match establish_connection(&dotenvy_env.database.url) {
        Ok(pool) => pool,
        Err(e) => {
            error!("Failed to establish connection to Postgres: {}", e);
            std::process::exit(1);
        }
    };

    info!("Postgres connection has been established");
}
