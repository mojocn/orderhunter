use clap::Parser;
use orderhunter::config::Config;
// use sqlx::mysql::MySqlPool;
use orderhunter::http;
use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();
    let config = Config::parse();

    let db_url = config.database_url.as_str();
    //let db = MySqlPool::connect(&config.database_url).await?;
    let db = SqlitePool::connect(db_url).await?;

    http::serve(config, db).await?;

    Ok(())
}
