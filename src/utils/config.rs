use std::env;

use dotenv::dotenv;
use tokio::sync::OnceCell;

#[derive(Debug)]
struct DatabaseConfig {
    url: String,
}

#[derive(Debug)]
struct ServerConfig {
    host: String,
    port: String,
}

#[derive(Debug)]
pub struct Config {
    server: ServerConfig,
    database: DatabaseConfig,
}

impl Config {
    pub fn db_url(&self) -> String {
        (&self.database.url).to_string()
    }
    pub fn server_host(&self) -> String {
        format!("{}:{}", &self.server.host, &self.server.port)
    }
}

pub static CONFIG: OnceCell<Config> = OnceCell::const_new();

async fn init_config() -> Config {
    dotenv().ok();

    let server_config = ServerConfig {
        host: env::var("HOST").unwrap_or_else(|_| String::from("127.0.0.1")),
        port: env::var("PORT").unwrap_or_else(|_| String::from("3000")),
    };

    let database_config = DatabaseConfig {
        url: env::var("DATABASE_URL").expect("DATABASE_URL must set"),
    };

    Config {
        server: server_config,
        database: database_config,
    }
}

pub async fn config() -> &'static Config {
    CONFIG.get_or_init(init_config).await
}
