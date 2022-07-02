use clap::Parser;
use orderhunter::config::Config;
use sqlx::mysql::MySqlPool;
use orderhunter::http;

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    dotenv::dotenv().ok();
    env_logger::init();
    let config = Config::parse();


    let db = MySqlPool::connect(&config.database_url).await?;

    http::serve(config, db).await?;

    Ok(())
}
