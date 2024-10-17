import { createInsertSchema, createSelectSchema } from "drizzle-zod";
import { z } from "astro:content";

import { items } from "@/db/schema";

export const itemsSelect = createSelectSchema(items);
export type Item = z.infer<typeof itemsSelect>;

export const itemsInsertSchema = createInsertSchema(items).extend({
  id: z.number().optional(),
});
