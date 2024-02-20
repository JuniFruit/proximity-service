use std::env;
#[derive(Debug)]
pub struct ServerConfig {
    pub port: String,
    pub mongo: String,
    pub redis: String,
}
impl ServerConfig {
    pub fn get() -> ServerConfig {
        let port = env::var("PORT").unwrap_or(8080.to_string());
        let mongo = env::var("MONGODB_URI").unwrap_or(String::from("mongodb://localhost:27017"));
        let redis = env::var("REDIS_URL").unwrap_or(String::from("localhost:5001"));

        ServerConfig { port, mongo, redis }
    }
}
