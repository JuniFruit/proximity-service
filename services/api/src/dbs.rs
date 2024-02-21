use crate::config::ServerConfig;
use bson::doc;
use mongodb::{error::Result, Client, Collection};
use serde::{Deserialize, Serialize};

pub struct DBConnections {
    pub mongo: MongoDb,
    pub redis_business: RedisBusiness,
}

impl DBConnections {
    pub async fn init(config: &ServerConfig) -> Result<DBConnections> {
        let mongo = MongoDb::connect(&config.mongo).await?;
        let redis_business = RedisBusiness::connect(&config.redis_business)?;

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
    pub async fn get_business_by_id(mongo: &MongoDb, id: u32) -> Result<BusinessData> {
        let businesses = mongo.get_businesses_collection();
        let business = businesses
            .find_one(Some(doc! { "id":  id }), None)
            .await?
            .unwrap();
        Ok(business)
    }
    pub async fn create_business(mongo: &MongoDb, mut data: BusinessData) -> Result<u64> {
        let businesses = mongo.get_businesses_collection();
        let count = businesses.count_documents(None, None).await.unwrap();
        let new_id = count + 1;
        data.id = Some(new_id);

        // Insert into the collection and extract the inserted_id value:
        businesses.insert_one(data, None).await?;
        Ok(new_id)
    }
    pub async fn update_business_by_id(mongo: &MongoDb, id: u32, data: BusinessData) -> Result<()> {
        let businesses = mongo.get_businesses_collection();
        let filter = doc! {"id": id};
        let updater = doc! {"$set": bson::to_bson(&data).unwrap().as_document().unwrap() };
        businesses
            .find_one_and_update(filter, updater, None)
            .await?;
        Ok(())
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
}

pub struct RedisBusiness {
    connection: redis::Connection,
}

impl RedisBusiness {
    pub fn connect(conn_str: &str) -> Result<RedisBusiness> {
        let client = redis::Client::open(conn_str).unwrap();
        let connection = client.get_connection().unwrap();
        Ok(RedisBusiness { connection })
    }
}
