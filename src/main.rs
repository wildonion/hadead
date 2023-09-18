



use log::info;
use tokio::time::Duration;
use ratelimiter;
use std::env;
use dotenv::dotenv;
use uuid::Uuid;
use env_logger::Env;
use wallexerr::*;


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

    /* --------------- */
    /* wallexerr tests */
    /* --------------- */
    let mut data = DataBucket{
        value: serde_json::to_string_pretty(&had_instance).unwrap(), /* json stringify of config had instance */ 
        signature: "".to_string(),
        signed_at: 0,
    };
    let stringify_data = serde_json::to_string_pretty(&data.value).unwrap();

    /* wallet operations */

    let mut contract = Contract::new_with_ed25519("0xDE6D7045Df57346Ec6A70DfE1518Ae7Fe61113f4");
    Wallet::save_to_json(&contract.wallet, "ed25519").unwrap();
    
    let signature_hex = Wallet::ed25519_sign(stringify_data.clone().as_str(), contract.wallet.ed25519_secret_key.as_ref().unwrap().as_str());
    
    let verify_res = Wallet::verify_ed25519_signature(signature_hex.clone().unwrap().as_str(), stringify_data.as_str(), contract.wallet.ed25519_public_key.clone().unwrap().as_str());

    let keypair = Wallet::retrieve_ed25519_keypair(
        /* 
            unwrap() takes the ownership of the type hence we must borrow 
            the type before calling it using as_ref() 
        */
        contract.wallet.ed25519_secret_key.unwrap().as_str()
    );

    /* fill the signature and signed_at fields if the signature was valid */
    if verify_res.is_ok(){
        data.signature = signature_hex.unwrap();
        data.signed_at = chrono::Local::now().timestamp_nanos();

        contract.data = Some(data);
    }

    
    // tokio::spawn(async move{

        let id = contract.wallet.ed25519_public_key.clone().unwrap();
        let is_limited = had_instance.check(&id).await.unwrap();
        
    // });


    Ok(())

}