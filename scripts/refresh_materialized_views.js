// run in background and dump logs to file. E.g: node refresh_materialized_view.js > refresh_views_logs.txt 2>&1 &
const { Client } = require("pg");
const parse = require("pg-connection-string").parse;

const connectionString = process.env.SUBSTREAMS_DATABASE;

if (!connectionString) {
  console.error("Please set the SUBSTREAMS_DATABASE environment variable");
  process.exit(1);
}

const config = parse(connectionString);

if (!config) {
  console.error("Error parsing connection string");
  process.exit(1);
} else {
  const client = new Client(config);

  client.connect();

  client.query("LISTEN refresh_mat_view");

  client.on("notification", (msg) => {
    if (msg.channel === "refresh_mat_view") {
      const materializedView = msg.payload;

      client
        .query(`REFRESH MATERIALIZED VIEW CONCURRENTLY ${materializedView}`)
        .then(() =>
          console.log(
            `Materialized view ${materializedView} refreshed successfully`
          )
        )
        .catch((err) =>
          console.error(
            `Error refreshing materialized view ${materializedView}:`,
            err
          )
        );
    }
  });

  client.on("error", (err) => {
    console.error("Error with PostgreSQL client:", err);
  });

  process.on("SIGINT", () => {
    client.end();
    process.exit();
  });
}
