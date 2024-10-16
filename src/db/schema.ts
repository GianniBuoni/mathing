import { integer, real, sqliteTable, text } from "drizzle-orm/sqlite-core";
import { createSelectSchema } from "drizzle-zod";
import { z } from "astro:content";

export const items = sqliteTable("items", {
  id: integer("id").notNull().unique().primaryKey(),
  item: text("item").notNull(),
  price: real("price").notNull(),
});

const itemsSelect = createSelectSchema(items);
export type Item = z.infer<typeof itemsSelect>;
