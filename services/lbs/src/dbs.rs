use crate::config::ServerConfig;
use redis::{FromRedisValue, RedisError, Value};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error};
pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub struct DBConnections {
    pub redis_business: RedisDB,
    pub redis_geo: RedisDB,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BusinessData {
    pub id: Option<u64>,
    pub name: String,
    pub stars: u8,
    pub r#type: String,
    pub lat: f32,
    pub lon: f32,
    pub opens_at: u8,
    pub closes_at: u8,
}

impl BusinessData {
    pub fn default() -> BusinessData {
        BusinessData {
            id: None,
            name: "null".to_string(),
            stars: 0,
            r#type: "null".to_string(),
            lat: 0.0,
            lon: 0.0,
            opens_at: 0,
            closes_at: 0,
        }
    }
    pub fn from_hashmap(map: HashMap<String, String>) -> Result<BusinessData> {
        if map.is_empty() {
            return Err("Hashmap cannot be empty".into());
        }
        let mut data = BusinessData::default();

        for (key, value) in map.iter() {
            let value = value.to_owned();
            match key.as_str() {
                "name" => data.name = value,
                "lon" => data.lon = value.parse::<f32>()?,
                "lat" => data.lat = value.parse::<f32>()?,
                "stars" => data.stars = value.parse::<u8>()?,
                "type" => data.r#type = value,
                "opensAt" => data.opens_at = value.parse::<u8>()?,
                "closesAt" => data.closes_at = value.parse::<u8>()?,
                _ => continue,
            }
        }

        Ok(data)
    }
    pub fn from_builk_redis(values: &[Value]) -> Result<BusinessData> {
        let mut map: HashMap<String, String> = HashMap::default();
        let mut key = "".to_string();
        for (i, val) in values.iter().enumerate() {
            let is_val = i % 2 != 0;
            if is_val {
                // in this case it would be actual value
                let v = match val {
                    redis::Value::Data(n) => {
                        String::from_utf8(n.to_owned()).unwrap_or("null".to_string())
                    }
                    _ => "null".to_string(),
                };
                map.insert(key.to_owned(), v);
            } else {
                // in this case it is field key
                key = match val {
                    redis::Value::Data(n) => {
                        String::from_utf8(n.to_owned()).unwrap_or("".to_string())
                    }
                    _ => "".to_string(),
                }
            }
        }
        let data = BusinessData::from_hashmap(map)?;
        Ok(data)
    }
}

impl FromRedisValue for BusinessData {
    fn from_redis_value(v: &redis::Value) -> redis::RedisResult<Self> {
        match v {
            redis::Value::Nil => Ok(BusinessData::default()),
            redis::Value::Bulk(values) => match BusinessData::from_builk_redis(values) {
                Ok(data) => Ok(data),
                Err(_) => Err(RedisError::from((
                    redis::ErrorKind::TypeError,
                    "Could not convert redis hash into BusinessData",
                ))),
            },
            _ => Err(RedisError::from((
                redis::ErrorKind::TypeError,
                "Could not construct BusinessData from single value",
            ))),
        }
    }
    fn from_redis_values(items: &[redis::Value]) -> redis::RedisResult<Vec<Self>> {
        let mut result: Vec<BusinessData> = vec![];
        for item in items {
            match BusinessData::from_redis_value(item) {
                Ok(data) => result.push(data),
                Err(e) => {
                    println!("Failed to convert value: {:?}", e)
                }
            }
        }

        Ok(result)
    }
}

impl DBConnections {
    pub async fn init(config: &ServerConfig) -> Result<DBConnections> {
        let redis_business = RedisDB::connect(&config.redis_business).await?;
        let redis_geo = RedisDB::connect(&config.redis_geo).await?;

        Ok(DBConnections {
            redis_business,
            redis_geo,
        })
    }
}
pub struct RedisDB {
    pub connection: redis::aio::MultiplexedConnection,
}

impl RedisDB {
    pub async fn connect(conn_str: &str) -> Result<RedisDB> {
        let client = redis::Client::open(conn_str).unwrap();
        let connection = client.get_multiplexed_async_connection().await.unwrap();

        Ok(RedisDB { connection })
    }
}
