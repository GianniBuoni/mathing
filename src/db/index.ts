import { drizzle } from "drizzle-orm/libsql";
import { createClient } from "@libsql/client";

// define drizzle adapter
const db = drizzle(
  createClient({
    url: "file:data.db",
  })
);

export default db;
