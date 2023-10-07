



use log::info;
use tokio::time::Duration;
use ratelimiter;
use std::env;
use dotenv::dotenv;
use uuid::Uuid;
use env_logger::Env;
use ratelimiter::generate_ed25519_contract;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>{

    env_logger::init_from_env(Env::default().default_filter_or("info")); /* is required to see info!() and error!() logs */
    dotenv::dotenv().unwrap();
    let redis_password = env::var("REDIS_PASSWORD").expect("⚠️ no redis password variable set");
    let redis_username = std::env::var("REDIS_USERNAME").expect("⚠️ no redis username variable set");
    let redis_host = std::env::var("REDIS_HOST").expect("⚠️ no redis host variable set");
    let redis_port = std::env::var("REDIS_PORT").expect("⚠️ no redis port variable set");
    let chill_zone_duration = std::env::var("CHILL_TIME").expect("⚠️ no chill time variable set").parse::<u64>().unwrap();

    let had_instance = ratelimiter::Config{
        redis_host,
        redis_port,
        redis_password: Some(redis_password),
        redis_username: None,
        chill_zone_duration /* default is 5 miliseconds */
    };


    let contract = generate_ed25519_contract(had_instance.clone());
    let id = contract.wallet.ed25519_public_key.clone().unwrap();
    let is_limited = had_instance.check(&id).await.unwrap();

    Ok(())

}