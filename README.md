


# 📛 Had

Redis Rate Limiter with Crypto Wallets as the Unique Identifier.

> Make sure that you've filled up the env vars inside `.env` file, also make sure you've installed the wallexerr crate using ```cargo add wallexerr```.

## 🚀 Run

```bash
cargo run --bin had
```

## 📦 Publish

```bash
cargo login
cargo publish --dry-run
cargo publish
```

## 🧪 Test

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
        id: None
    };

    hadead_instance

});

pub async fn api(){

    let hadead = HADEAD.clone();
    if let Err(why) = hadead.check(hadead.id.as_ref().unwrap()).await{
        eprintln!("hadead redis error because: {}", why.to_string());
    }

}
```