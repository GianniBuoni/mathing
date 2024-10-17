import { integer, real, sqliteTable, text } from "drizzle-orm/sqlite-core";
import { createInsertSchema, createSelectSchema } from "drizzle-zod";
import { z } from "astro:content";

export const items = sqliteTable("items", {
  id: integer("id").notNull().unique().primaryKey({ autoIncrement: true }),
  item: text("item").notNull(),
  price: real("price").notNull(),
});

export const itemsSelect = createSelectSchema(items);
export type Item = z.infer<typeof itemsSelect>;

export const itemsInsertSchema = createInsertSchema(items).extend({
  id: z.number().optional(),
});
