import { integer, real, sqliteTable, text } from "drizzle-orm/sqlite-core";

export const items = sqliteTable("items", {
  id: integer("id").notNull().unique().primaryKey({ autoIncrement: true }),
  item: text("item").notNull(),
  price: real("price").notNull(),
});
