import { drizzle } from "drizzle-orm/libsql";
import { createClient } from "@libsql/client";
import { items } from "./schema";

// define drizzle adapter
const db = drizzle(
  createClient({
    url: "file:data.db",
  }),
  { schema: { items } },
);

export default db;
