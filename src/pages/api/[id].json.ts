import type { APIRoute } from "astro";

import db from "@/db";
import { items } from "@/db/schema";
import type { Item } from "@/db/schema";
import { eq } from "drizzle-orm";

export const PATCH: APIRoute = async ({ params, props }) => {
  const id = Number(params);
  const body = (await props.json()) as Item;
  try {
    if (!body) throw new Error("No body was provided");

    const selectItem = await db.query.items.findFirst({
      where: eq(items.id, id),
    });
    if (!selectItem) throw new Error("Item ID could not be found");
    const editedItem = await db
      .update(items)
      .set({
        item: body.item,
        price: body.price,
      })
      .where(eq(items.id, selectItem.id));

    return new Response(JSON.stringify(editedItem), { status: 200 });
  } catch (e) {
    console.log(e);
    return new Response("There was an error updating the item");
  }
};

export const DELETE: APIRoute = async ({ params }) => {
  const id = Number(params);

  try {
    const selectItem = await db.query.items.findFirst({
      where: eq(items.id, id),
    });
    if (!selectItem) throw new Error("Item ID could not be found");
    await db.delete(items).where(eq(items.id, selectItem.id));

    return new Response("Item deleted", { status: 200 });
  } catch (e) {
    console.log(e);
    return new Response("There was an error deleting the item");
  }
};
