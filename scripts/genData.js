const redisDriver = require("ioredis");
const readline = require("readline");
const mongoDriver = require("mongodb");
const fs = require("fs");
const faker = require("@faker-js/faker").allFakers;

const TYPES = ["restaurant", "car-wash", "cafe", "hotel", "shop"];
const maxBusinessPerCity = 70;

const rnd = (num) => {
  return Math.floor(Math.random() * num);
};

function randomGeo(center, radius) {
  var y0 = center.latitude;
  var x0 = center.longitude;
  var rd = radius / 111300;

  var u = Math.random();
  var v = Math.random();

  var w = rd * Math.sqrt(u);
  var t = 2 * Math.PI * v;
  var x = w * Math.cos(t);
  var y = w * Math.sin(t);

  return {
    latitude: y + y0,
    longitude: x + x0,
  };
}

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

const generateData = async (line, clients) => {
  const [_, __, latitude, longitude] = line.split(",");
  let created = 0;
  const center = {
    latitude: parseFloat(latitude?.replaceAll('"', "")),
    longitude: parseFloat(longitude?.replaceAll('"', "")),
  };

  for (let i = 0; i < maxBusinessPerCity; i++) {
    try {
      const { latitude: lat, longitude: lon } = randomGeo(center, 12000);
      if (!lat || !lon) continue;
      const zipCode = faker.en.location.zipCode();
      const stars = faker.en.number.int({ min: 2, max: 5 });
      const opensAt = faker.en.number.int({ min: 6, max: 12 });
      const closesAt = faker.en.number.int({ min: 18, max: 24 });
      const averagePrice = faker.en.number.int({ min: 5, max: 25 });
      const description = faker.en.lorem.paragraph();
      const email = faker.en.internet.email();
      const phone = faker.en.phone.number().toString();
      const name = faker.en.company.name();
      const id = faker.en.seed();

      const record = Object.setPrototypeOf(
        {
          id,
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
      created++;
    } catch (error) {
      handleErr(error);
      continue;
    }
  }
  return created;
};

const generateRecords = async () => {
  const clients = await connectToAllDbs();
  const maxRecords = 47868;
  let ind = 0;
  let created = 0;

  const fileStream = fs.createReadStream("./worldcities.csv");
  fileStream.le;

  let lineReader = require("readline").createInterface({
    input: fs.createReadStream("./worldcities.csv"),
  });

  for await (const line of lineReader) {
    if (ind % 50 == 0) {
      const done = Math.trunc((ind / maxRecords) * 100);
      console.clear();
      console.log("Migrating data... \nLines processed: " + ind);
      console.log("Item recorded: " + created);
      console.log("[" + "#".repeat(done) + "_".repeat(100 - done) + "]");
      console.log("Done: " + done + "%");
    }

    try {
      created += await generateData(line, clients);
      ind++;
    } catch (error) {
      handleErr(error);
      ind++;
      continue;
    }
  }

  console.clear();
  console.log("Finished migrating data. Entries created: " + created);
  process.exit(1);
};

function handleErr(error) {
  console.log("Failed to create entry. Reason: " + error.message);
}

generateRecords();
