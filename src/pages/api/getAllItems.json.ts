import type { APIRoute } from "astro";
import { asc } from "drizzle-orm";

import db from "@/db";
import { items } from "@/db/schema";

export const GET: APIRoute = async () => {
  const allItems = await db.select().from(items).orderBy(asc(items.item));
  return new Response(JSON.stringify(allItems));
};
