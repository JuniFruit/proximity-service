const redisDriver = require("ioredis");
const mongoDriver = require("mongodb");
const faker = require("@faker-js/faker").allFakers;

const TYPES = ["restaurant", "car-wash", "cafe", "hotel", "shop"];

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
  const clients = await connectToAllDbs();
  const maxRecords = 1600000;

  let ind = 0;
  for (let i = 0; i < maxRecords; i++) {
    try {
      if (ind % 5000 === 0) {
        const done = Math.trunc((ind / maxRecords) * 100);
        console.clear();
        console.log("Migrating data... Items recorded: " + ind);
        console.log("[" + "#".repeat(done) + "_".repeat(100 - done) + "]");
        console.log("Done: " + done + "%");
      }
      const zipCode = faker.en.location.zipCode;
      const stars = faker.en.number.int({ min: 2, max: 5 });
      const opensAt = faker.en.number.int({ min: 6, max: 12 });
      const closesAt = faker.en.number.int({ min: 18, max: 24 });
      const averagePrice = faker.en.number.int({ min: 5, max: 25 });
      const description = faker.en.lorem.paragraph();
      const email = faker.en.internet.email();
      const phone = faker.en.phone.number().toString();
      const lat = faker.en.location.latitude();
      const lon = faker.en.location.longitude();
      const name = faker.en.company.name();

      const record = Object.setPrototypeOf(
        {
          id: ind,
          zipCode,
          name,
          stars,
          type: TYPES[rnd(TYPES.length)],
          lat,
          lon,
          opensAt,
          closesAt,
          averagePrice,
          description,
          email,
          phone,
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
          "zipCode",
          zipCode,
          "name",
          name,
          "stars",
          stars,
          "lat",
          lat,
          "lon",
          lon,
          "type",
          record.type,
          "averagePrice",
          averagePrice,
          "closesAt",
          closesAt,
          "opensAt",
          opensAt,
          "description",
          description,
          "email",
          email,
          "phone",
          phone,
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
  console.log("Finished migrating data. Entries created: " + ind);
};

function handleErr(error) {
  console.error("Failed to add record" + ". Reason: " + error.message);
}

generateRecords();
