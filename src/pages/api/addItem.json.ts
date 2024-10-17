import type { APIRoute } from "astro";
import { items, itemsInsertSchema } from "@/db/schema";
import type { Item } from "@/db/schema";
import db from "@/db";

export const POST: APIRoute = async ({ props }) => {
  const body = (await props.json()) as Item;
  try {
    const validate = itemsInsertSchema.safeParse(body);
    if (!validate) throw new Error("Passed props don't match insert schema");

    const newItem = await db.insert(items).values({
      item: body.item,
      price: body.price,
    });

    return new Response(JSON.stringify(newItem), { status: 201 });
  } catch (e) {
    console.error(e);
    return new Response("There was an issue inserting the item");
  }
};
