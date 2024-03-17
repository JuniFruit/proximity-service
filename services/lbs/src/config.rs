use std::env;
#[derive(Debug)]
pub struct ServerConfig {
    pub port: String,
    pub redis_business: String,
    pub redis_geo: String,
}
impl ServerConfig {
    pub fn get() -> ServerConfig {
        let port = env::var("PORT").unwrap_or(8081.to_string());
        let redis_business =
            env::var("REDIS_BUSINESS_URI").unwrap_or(String::from("redis://localhost:6378"));
        let redis_geo = env::var("REDIS_GEO_URI").unwrap_or(String::from("redis://localhost:6377"));

        ServerConfig {
            port,
            redis_business,
            redis_geo,
        }
    }
}
