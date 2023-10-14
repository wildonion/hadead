

use log::info;
use std::env;
use env_logger::Env;
use log::error;
use serde::{Serialize, Deserialize};
use redis::RedisError;
use redis::FromRedisValue;
use redis::JsonAsyncCommands;
use redis::cluster::ClusterClient;
use redis::AsyncCommands; //// this trait is required to be imported in here to call set() methods on the cluster connection
use redis::RedisResult;
use std::collections::HashMap;
use wallexerr::*;



#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config{
    pub redis_password: Option<String>,
    pub redis_username: Option<String>,
    pub redis_host: String,
    pub redis_port: String,
    pub chill_zone_duration_in_seconds: u64,
    pub id: Option<String>
}


impl Config{
    
    pub fn new(redis_password: &str, 
        redis_username: &str, 
        redis_host: &str, 
        redis_port: &str, 
        chill_zone_duration_in_seconds: u64) -> Self{

        let hadead_instance = serde_json::json!({
            "redis_username": redis_username,
            "redis_host": redis_host,
            "redis_port": redis_port,
            "chill_zone_duration_in_seconds": chill_zone_duration_in_seconds
        });

        let contract = generate_ed25519_contract(hadead_instance.as_str().unwrap());
        let id = contract.wallet.ed25519_public_key.clone().unwrap();

        Config { 
            redis_password: Some(redis_password.to_string()), 
            redis_username: Some(redis_username.to_string()), 
            redis_host: redis_host.to_string(), 
            redis_port: redis_port.to_string(),
            chill_zone_duration_in_seconds,
            id: Some(id)
        }

    }

    pub async fn check(&self, peer_unique_identifire: &str) -> Result<bool, RedisError>{

        let Config { 
            redis_password, 
            redis_username, 
            redis_host, 
            redis_port, 
            chill_zone_duration_in_seconds,
            id
        } = self; /* unpacking self */

        /* 
            since self is behind a shared reference means there is borrow of self is exists
            thus we can't move out of it or take the ownership of self, which this is happening 
            by unwrap() method, hence we must use as_ref() method to not to lose the ownership
            of self

            based on the passed in username, password and port we got the following logic
            to create the redis connection url
        */
        let redis_conn_url = if self.redis_password.is_some(){
            format!("redis://:{}@{}:{}", self.redis_password.as_ref().unwrap(), self.redis_host, self.redis_port)
        } else if self.redis_password.is_some() && self.redis_username.is_some(){
            format!("redis://{}:{}@{}:{}", self.redis_username.as_ref().unwrap(), self.redis_password.as_ref().unwrap(), self.redis_host, self.redis_port)
        } else{
            format!("redis://{}:{}", self.redis_host, self.redis_port)
        };

        let get_redis_client = redis::Client::open(redis_conn_url.as_str());
        let Ok(redis_client) = get_redis_client else{
            return Err(get_redis_client.unwrap_err());
        };

        
        let get_redis_conn = redis_client.get_async_connection().await;
        let Ok(mut redis_conn) = get_redis_conn else{
            return Err(get_redis_conn.err().unwrap());
        };
        
        
        /* rate limiter based on peer_unique_identifire */
        
        let chill_zone_duration_in_seconds = self.chill_zone_duration_in_seconds * 1000; //// 5 seconds chillzone
        let now = chrono::Local::now().timestamp_millis() as u64;
        let mut is_rate_limited = false;
        
        let redis_result_hadead: RedisResult<String> = redis_conn.get(peer_unique_identifire).await;
        let mut redis_hadead = match redis_result_hadead{
            Ok(data) => {
                let rl_data = serde_json::from_str::<HashMap<String, u64>>(data.as_str()).unwrap();
                rl_data
            },
            Err(e) => {
                let empty_hadead = HashMap::<String, u64>::new();
                let rl_data = serde_json::to_string(&empty_hadead).unwrap();
                let _: () = redis_conn.set(peer_unique_identifire, rl_data).await.unwrap();
                HashMap::new()
            }
        };
        
        if let Some(last_used) = redis_hadead.get(peer_unique_identifire){
            if now - *last_used < chill_zone_duration_in_seconds{
                is_rate_limited = true;
            }
        }
        
        
        if is_rate_limited{
            
            error!("⛔ Access Denied, ☕ chill for {:?} seconds", (chill_zone_duration_in_seconds/1000) as u64);
            Ok(true) // rate limited
            
        
        } else{
            
            /* updating the last rquest time */
            // this will be used to handle shared state between clusters
            redis_hadead.insert(peer_unique_identifire.to_string(), now); //// updating the redis rate limiter map
            let rl_data = serde_json::to_string(&redis_hadead).unwrap();
            let _: () = redis_conn.set(peer_unique_identifire, rl_data).await.unwrap(); //// writing to redis ram

            info!("🔓 Access Granted");
            Ok(false) // not rate limited
            
        }


    
    }

}


fn generate_ed25519_contract(hadead_instance: &str) -> wallexerr::Contract{

    /* --------------------------------------- */
    /*          wallexerr operations           */
    /* --------------------------------------- */
    let mut data = DataBucket{
        value: hadead_instance.to_string(), /* json stringify of config had instance */ 
        signature: "".to_string(),
        signed_at: 0,
    };
    let stringify_data = serde_json::to_string_pretty(&data.value).unwrap();

    let mut contract = Contract::new_with_ed25519("");
    Wallet::save_to_json(&contract.wallet, "ed25519").unwrap();
    
    let signature_hex = Wallet::ed25519_sign(stringify_data.clone().as_str(), contract.wallet.ed25519_secret_key.as_ref().unwrap().as_str());
    
    let verify_res = Wallet::verify_ed25519_signature(signature_hex.clone().unwrap().as_str(), stringify_data.as_str(), contract.wallet.ed25519_public_key.clone().unwrap().as_str());

    let keypair = Wallet::retrieve_ed25519_keypair(
        /* 
            unwrap() takes the ownership of the type hence we must borrow 
            the type before calling it using as_ref() 
        */
        contract.wallet.ed25519_secret_key.clone().unwrap().as_str()
    );

    /* fill the signature and signed_at fields if the signature was valid */
    if verify_res.is_ok(){
        data.signature = signature_hex.unwrap();
        data.signed_at = chrono::Local::now().timestamp_nanos();

        contract.data = Some(data);
    }

    contract

}