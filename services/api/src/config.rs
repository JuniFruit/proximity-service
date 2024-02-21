use std::env;
#[derive(Debug)]
pub struct ServerConfig {
    pub port: String,
    pub mongo: String,
    pub redis_business: String,
}
impl ServerConfig {
    pub fn get() -> ServerConfig {
        let port = env::var("PORT").unwrap_or(8080.to_string());
        let mongo = env::var("MONGODB_URI")
            .unwrap_or(String::from("mongodb://root:password@localhost:27017"));
        let redis_business =
            env::var("REDIS_BUSINESS_URI").unwrap_or(String::from("redis://localhost:6378"));

        ServerConfig {
            port,
            mongo,
            redis_business,
        }
    }
}
