import SQL from "sql-template-strings";
import { Pool, PoolClient } from "pg";
import { Command } from "commander";
import { exec } from "child_process";

const SCHEMA_PREFIX = "dcl";
const INITIAL_SCHEMA = `${SCHEMA_PREFIX}1`;

async function getLatestSchema(client: PoolClient): Promise<string | null> {
  try {
    const query = `
      SELECT information.schema_name
      FROM information_schema.schemata AS information
      WHERE schema_name LIKE $1::text || '%'
      ORDER BY CAST(SUBSTRING(information.schema_name FROM 'dcl([0-9]+)') AS INTEGER) DESC
      LIMIT 1;
    `;

    const result = await client.query(query, [SCHEMA_PREFIX]);

    if (result.rowCount > 0) {
      const latestSchema = result.rows[0].schema_name;
      return latestSchema;
    } else {
      return null; // No matching schema found
    }
  } catch (error) {
    console.error("Error retrieving latest schema:", error);
    return null;
  }
}

function incrementSchema(schema: string) {
  // Extract the number from the input string
  const schemaNumber = schema.match(/\d+/)?.[0];
  if (schemaNumber) {
    const number = parseInt(schemaNumber);
    const incrementedNumber = number + 1;
    // Replace the number in the string with the incremented value
    return schema.replace(/\d+$/, `${incrementedNumber}`);
  }
  throw new Error("Invalid schema");
}

export async function createNewSchema(client: PoolClient, network: string) {
  const latestSchema = await getLatestSchema(client);
  const newSchema = incrementSchema(latestSchema || INITIAL_SCHEMA);
  console.log("Latest schema in the db: ", latestSchema);
  console.log("New schema created: ", newSchema);
  try {
    await client.query(SQL`CREATE SCHEMA IF NOT EXISTS `.append(newSchema));
    await client.query(
      `INSERT INTO substreams.deployments (schema, network) VALUES ($1, $2)`,
      [newSchema, network]
    );
    const createCursorQuery = SQL`
        CREATE TABLE `
      .append(newSchema)
      .append(
        SQL`.cursors (
          id text PRIMARY KEY,
          cursor text,
          block_num bigint,
          block_id text
      );
      
      -- Indices -------------------------------------------------------

      CREATE UNIQUE INDEX cursor_pk ON `
          .append(newSchema)
          .append(SQL`.cursors(id text_ops)`)
      );
    await client.query(createCursorQuery);
    return newSchema;
  } catch (error) {
    console.error(error);
    throw Error("Error creating new schema");
  }
  return newSchema;
}

const grantAccess = async (client: PoolClient, newSchema: string) => {
  await client.query(
    SQL`GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA `
      .append(newSchema)
      .append(` TO dappssubstreamsuser;`)
  );
  await client.query(
    SQL`GRANT USAGE ON SCHEMA `
      .append(newSchema)
      .append(` TO dappssubstreamsuser;`)
  );
};

// Function to run the sink command
function runSinkSetupCommand(
  db: string,
  callback: (error: Error | null, stdout: string, stderr: string) => void
) {
  exec(
    `../substreams-sink-postgres setup '${db}' ../schema.sql`,
    (error, stdout, stderr) => {
      callback(error, stdout, stderr);
    }
  );
}

async function main() {
  // Define command-line options
  const program = new Command();
  program.option("-d, --db <connectionString>", "Database connection string");
  program.option(
    "-n, --network <network>",
    "Network that the schema belongs to"
  );

  // Parse the command-line arguments
  program.parse(process.argv);
  const options = program.opts();

  // Retrieve the database connection string from the command-line options
  const dbConnectionString = options.db;
  if (!dbConnectionString) {
    throw new Error(
      "Database connection string is required. E.g: --db postgres://user:password@localhost:5432/db_name"
    );
  }
  // Retrieve the network string from the command-line options
  const network = options.network;
  console.log("network: ", network);
  if (!network) {
    throw new Error("Network is required. E.g: --network polygon");
  }
  console.log("dbConnectionString: ", dbConnectionString);

  const pool = new Pool({ connectionString: dbConnectionString });
  const client = await pool.connect();

  //   const latestSchema = await getLatestSchema(client);

  const newSchema = await createNewSchema(client, network);

  runSinkSetupCommand(
    dbConnectionString,
    async (error: Error | null, stdout: string, stderr: string) => {
      if (error) {
        console.error("Error running sink command:", error);
      }

      await grantAccess(client, newSchema);

      client.release();
      // Close the database connection pool
      await pool.end();

      console.log("Sink command executed successfully:", stdout);
    }
  );
}

main().catch((error) => {
  console.error("Script execution error:", error);
  process.exit(1);
});
