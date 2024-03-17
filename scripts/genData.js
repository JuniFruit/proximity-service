const fs = require("fs");
const readline = require("readline");
const redisDriver = require("ioredis");
const mongoDriver = require("mongodb");

const COUNTRIES_PATH = "./allCountries.txt";
const NAMES = [
  "Joes",
  "SteakHouse",
  "Market",
  "Washer",
  "Donalds",
  "Trader",
  "San Morto",
  "Japanese",
  "Chinese",
  "Restaurant",
  "Suns",
  "Williams",
  "Lakes",
  "Fake",
];
const TYPES = ["restaurant", "car-wash", "diner", "motel", "hotel"];

const rnd = (num) => {
  return Math.floor(Math.random() * num);
};

const connectToAllDbs = async () => {
  const redisGeo1 = new redisDriver({
    host: process.env.MAIN_HOST,
    port: process.env.REDIS_GEO_PORT_1,
  });
  const redisBusiness1 = new redisDriver({
    host: process.env.MAIN_HOST,
    port: process.env.REDIS_BUSINESS_INFO_PORT_1,
  });
  const mongo1 = new mongoDriver.MongoClient(
    `mongodb://${process.env.MONGO_INITDB_ROOT_USERNAME}:${process.env.MONGO_INITDB_ROOT_PASSWORD}@${process.env.MAIN_HOST}:${process.env.MONGO_MAIN_PORT_1}`,
  );
  await mongo1.connect();
  const mongoDB = mongo1.db("main");

  return {
    redisGeo1,
    redisBusiness1,
    mongoDB,
  };
};

const generateRecords = async () => {
  const countriesStream = fs.createReadStream(COUNTRIES_PATH);
  const clients = await connectToAllDbs();
  const maxRecords = 1553663;
  const rl = readline.createInterface({
    input: countriesStream,
    crlfDelay: Infinity,
  });

  let ind = 0;
  for await (const line of rl) {
    try {
      if (ind % 5000 === 0) {
        const done = (ind / maxRecords) * 100;
        console.clear();
        console.log("Migrating data... Items recorded: " + ind);
        console.log("Done: " + Math.trunc(done) + "%");
      }
      const splitted = line.split("\t");
      const countryCode = splitted[0];
      const zipCode = splitted[1];
      const street = splitted[2];
      const city = splitted[3];
      const lat = +splitted[splitted.length - 3];
      const lon = +splitted[splitted.length - 2];

      const record = Object.setPrototypeOf(
        {
          id: ind,
          countryCode,
          zipCode,
          street,
          name: NAMES[rnd(NAMES.length)] + " " + NAMES[rnd(NAMES.length)],
          stars: rnd(5),
          type: TYPES[rnd(TYPES.length)],
          city,
          lat,
          lon,
        },
        null,
      );
      const promises = [
        clients.redisGeo1.call(
          "GEOADD",
          "world",
          record.lon,
          record.lat,
          record.id,
        ),
        clients.redisBusiness1.call(
          "HSET",
          record.id,
          "country_code",
          record.countryCode,
          "zip_code",
          record.zipCode,
          "name",
          record.name,
          "stars",
          record.stars,
          "street",
          record.street,
          "lat",
          record.lat,
          "lon",
          record.lon,
          "city",
          record.city,
          "type",
          record.type,
        ),
        clients.mongoDB.collection("businesses").insertOne(record),
      ];
      await Promise.all(promises);
      ind++;
    } catch (error) {
      handleErr(error);
      ind++;
      continue;
    }
  }
};

function handleErr(error) {
  console.error("Failed to add record" + ". Reason: " + error.message);
}

generateRecords();
