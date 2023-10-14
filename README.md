


# 📛 Hadead

Redis Rate Limiter based on [wallexerr](https://crates.io/crates/wallexerr) **ed25519** public key as the unique identifier.

## 🚀 Run in Local

```bash
cargo run --bin hadead
```

## 🧪 Test

> `hadead.contract` is a contract data contains the wallet info.

```rust
use hadead::*;
use once_cell::sync::Lazy;

pub static HADEAD: Lazy<Config> = Lazy::new(||{

    let redis_password = "REDIS_PASSWORD".to_string();
    let redis_username = "REDIS_USERNAME".to_string();
    let redis_host = "REDIS_HOST".to_string();
    let redis_port = "REDIS_PORT".to_string();
    let chill_zone_duration_in_seconds = 5;

    let hadead_instance = hadead::Config{
        redis_host,
        redis_port,
        redis_password: Some(redis_password),
        redis_username: None,
        chill_zone_duration_in_seconds, /* default is 5 miliseconds */
        id: None,
        contract: None,
    };

    hadead_instance

});

pub async fn api() -> Result<actix_web::HttpResponse, actix_web::Error>{

    let hadead = HADEAD.clone();
    println!("hadead contract info: {:?}", hadead.contract.as_ref().unwrap());

    let check_rate_limited = hadead.check(hadead.id.as_ref().unwrap()).await;
    
    let Ok(flag) = check_rate_limited else{
        
        let why = check_rate_limited.unwrap_err();
        return Ok(
            HttpResponse::NotAcceptable().json(why.to_string())
        );
    };

    if flag{

        // rate limited

        return Ok(
            HttpResponse::NotAcceptable().json("rate limited")
        );

    } else{

        // other api logic
        // ...

        return Ok(
            HttpResponse::Ok().json("json data")
        );

    }

}
```
