


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
env_logger::init_from_env(Env::default().default_filter_or("info")); /* is required to see info!() and error!() logs */
let redis_password = "REDIS_PASSWORD".to_string();
let redis_username = "REDIS_USERNAME".to_string();
let redis_host = "REDIS_HOST".to_string();
let redis_port = "REDIS_PORT".to_string();
let chill_zone_duration_in_seconds = 5;

let had_instance = self::Config{
    redis_host,
    redis_port,
    redis_password: Some(redis_password),
    redis_username: None,
    chill_zone_duration_in_seconds, /* default is 5 miliseconds */
    id: None
};

let id = had_instance.id.as_ref().unwrap();

info!("💳 generated unique peer identifier (look inside `wallexerr-keys` folder): [{}]", id);

had_instance.check(id).await
```