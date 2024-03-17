use crate::{config::ServerConfig, response::Result};
use bson::doc;
use mongodb::{Client, Collection};
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct DBConnections {
    pub mongo: MongoDb,
    pub redis_business: RedisBusiness,
}

impl DBConnections {
    pub async fn init(config: &ServerConfig) -> Result<DBConnections> {
        let mongo = MongoDb::connect(&config.mongo).await?;
        let redis_business = RedisBusiness::connect(&config.redis_business).await?;

        Ok(DBConnections {
            mongo,
            redis_business,
        })
    }
}

#[derive(Clone)]
pub struct MongoDb {
    client: Client,
}

impl MongoDb {
    pub async fn connect(mongo_uri: &str) -> Result<MongoDb> {
        let client = Client::with_uri_str(mongo_uri)
            .await
            .expect("Failed to connect to Mongo Db");
        Ok(MongoDb { client })
    }
    pub fn get_businesses_collection(&self) -> Collection<BusinessData> {
        self.client.database("main").collection("businesses")
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BusinessData {
    pub id: Option<u64>,
    pub country_code: String,
    pub zip_code: String,
    pub street: String,
    pub name: String,
    pub stars: u8,
    pub r#type: String,
    pub city: String,
    pub lat: f32,
    pub lon: f32,
}

impl BusinessData {
    pub fn default() -> BusinessData {
        BusinessData {
            id: None,
            country_code: "EN".to_string(),
            zip_code: "0".to_string(),
            street: "null".to_string(),
            name: "null".to_string(),
            stars: 0,
            r#type: "null".to_string(),
            city: "null".to_string(),
            lat: 0.0,
            lon: 0.0,
        }
    }
    async fn get_business_by_id_mongo(mongo: &MongoDb, id: u32) -> Result<Option<BusinessData>> {
        let businesses = mongo.get_businesses_collection();
        let business = businesses.find_one(Some(doc! { "id":  id }), None).await?;
        Ok(business)
    }
    async fn create_business_mongo(mongo: &MongoDb, data: &mut BusinessData) -> Result<u64> {
        let businesses = mongo.get_businesses_collection();
        let count = businesses.count_documents(None, None).await?;
        // We simulate auto incrementing id instead of Mongo ObjectId
        // It's used for simplicity and debugging purposes, it's not a good idea to use it in prod
        let new_id = count + 1;
        data.id = Some(new_id);

        businesses.insert_one(data, None).await?;
        Ok(new_id)
    }
    async fn update_business_by_id_mongo(
        mongo: &MongoDb,
        id: u32,
        data: &BusinessData,
    ) -> Result<()> {
        let businesses = mongo.get_businesses_collection();
        let filter = doc! {"id": id};
        let updater = doc! {"$set": bson::to_bson(&data).unwrap().as_document().unwrap() };
        businesses
            .find_one_and_update(filter, updater, None)
            .await?;
        Ok(())
    }
    pub async fn update_business_by_id(
        dbs: &mut DBConnections,
        id: u32,
        mut data: BusinessData,
    ) -> Result<()> {
        BusinessData::update_business_by_id_mongo(&dbs.mongo, id, &data).await?;
        data.id = Some(id as u64);
        BusinessData::cache_business_data(&mut dbs.redis_business, &data).await?;
        Ok(())
    }

    pub async fn create_business(dbs: &mut DBConnections, mut data: BusinessData) -> Result<u64> {
        let inserted_id = BusinessData::create_business_mongo(&dbs.mongo, &mut data).await?;
        BusinessData::cache_business_data(&mut dbs.redis_business, &data).await?;
        Ok(inserted_id)
    }

    pub async fn get_business_by_id(
        dbs: &mut DBConnections,
        id: u32,
    ) -> Result<Option<BusinessData>> {
        let cached = BusinessData::get_business_by_id_redis(&mut dbs.redis_business, id).await?;

        if cached.is_some() {
            return Ok(cached);
        }
        let from_mongo = BusinessData::get_business_by_id_mongo(&dbs.mongo, id).await?;

        if from_mongo.is_some() {
            BusinessData::cache_business_data(
                &mut dbs.redis_business,
                &from_mongo.clone().unwrap(),
            )
            .await?;
        }

        Ok(from_mongo)
    }

    async fn cache_business_data(redis: &mut RedisBusiness, data: &BusinessData) -> Result<()> {
        let id = data.id;
        if id.is_none() {
            return Err("Failed to cache, id does not exist".into());
        }
        redis
            .set_hash(
                id.unwrap().to_string().as_str(),
                data.clone().into_hashmap()?,
            )
            .await?;
        Ok(())
    }
    async fn get_business_by_id_redis(
        redis: &mut RedisBusiness,
        id: u32,
    ) -> Result<Option<BusinessData>> {
        let data = redis.get_hash_by_id(id).await?;
        match BusinessData::from_hashmap(data) {
            Ok(mut constructed) => {
                constructed.id = Some(id as u64);
                Ok(Some(constructed))
            }
            Err(e) => {
                println!("{:?}", e);
                Ok(None)
            }
        }
    }

    pub fn from_value(value: &serde_json::Value) -> serde_json::Result<BusinessData> {
        match serde_json::from_value(value.clone()) {
            Ok(res) => Ok(res),
            Err(e) => {
                println!("{:?}", e);
                Err(e)
            }
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
                "street" => data.street = value,
                "stars" => data.stars = value.parse::<u8>()?,
                "zip_code" => data.zip_code = value,
                "country_code" => data.country_code = value,
                "type" => data.r#type = value,
                "city" => data.city = value,
                _ => continue,
            }
        }

        Ok(data)
    }

    pub fn into_hashmap(self) -> Result<HashMap<String, String>> {
        let value = serde_json::to_value(self)?;
        let mut map: HashMap<String, String> = HashMap::default();
        for (k, v) in value.as_object().unwrap().into_iter() {
            if v.is_f64() {
                map.insert(k.to_owned(), v.as_f64().unwrap().to_string());
            } else if v.is_u64() {
                map.insert(k.to_owned(), v.as_u64().unwrap().to_string());
            } else if v.is_string() {
                map.insert(k.to_owned(), v.as_str().unwrap().to_string());
            } else {
                continue;
            }
        }
        Ok(map)
    }
}

pub struct RedisBusiness {
    connection: redis::aio::MultiplexedConnection,
}

impl RedisBusiness {
    pub async fn connect(conn_str: &str) -> Result<RedisBusiness> {
        let client = redis::Client::open(conn_str).unwrap();
        let connection = client.get_multiplexed_async_connection().await.unwrap();

        Ok(RedisBusiness { connection })
    }

    pub async fn get_hash_by_id(&mut self, id: u32) -> Result<HashMap<String, String>> {
        let data: HashMap<String, String> = self.connection.hgetall(id).await?;
        Ok(data)
    }

    pub async fn set_hash(&mut self, key: &str, map: HashMap<String, String>) -> Result<()> {
        let mut values: Vec<(&str, &str)> = Vec::with_capacity(map.len());

        for (key, value) in map.iter() {
            if key == "id" {
                continue;
            }
            values.push((key.as_str(), value.as_str()));
        }

        self.connection.hset_multiple(key, &values).await?;
        Ok(())
    }
}
