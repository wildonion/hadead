



use log::info;
use tokio::time::Duration;
use ratelimiter;
use std::env;
use dotenv::dotenv;
use uuid::Uuid;
use env_logger::Env;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>{

    env_logger::init_from_env(Env::default().default_filter_or("info")); /* is required to see info!() and error!() logs */
    dotenv::dotenv().unwrap();
    let redis_password = env::var("REDIS_PASSWORD").expect("⚠️ no redis password variable set");
    let redis_username = std::env::var("REDIS_USERNAME").expect("⚠️ no redis username variable set");
    let redis_host = std::env::var("REDIS_HOST").expect("⚠️ no redis host variable set");
    let redis_port = std::env::var("REDIS_PORT").expect("⚠️ no redis port variable set");

    let had_instance = ratelimiter::Config{
        redis_host,
        redis_port,
        redis_password: Some(redis_password),
        redis_username: None,
        chill_zone_duration: 5_000 as u64
    };


    
    // tokio::spawn(async move{

        let id = "e2b0f1ad-db86-4126-87ca-d84d10e46343".to_string();
        let is_limited = had_instance.check(&id).await.unwrap();

        info!("is limited {}", is_limited);
        
    // });


    Ok(())

}