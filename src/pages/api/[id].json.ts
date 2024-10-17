import type { APIRoute } from "astro";

import db from "@/db";
import { items } from "@/db/schema";
import type { Item } from "@/db/types";
import { eq } from "drizzle-orm";

export const PATCH: APIRoute = async ({ params, request }) => {
  const id = Number(params.id);
  const body = (await request.json()) as Item;
  try {
    if (!body) return new Response("No body was provided", { status: 404 });

    const selectItem = await db.query.items.findFirst({
      where: eq(items.id, id),
    });
    if (!selectItem)
      return new Response("Item ID could not be found", { status: 404 });
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
    return new Response("There was an error updating the item", {
      status: 400,
    });
  }
};

export const DELETE: APIRoute = async ({ params }) => {
  const id = Number(params.id);

  try {
    const selectItem = await db.query.items.findFirst({
      where: eq(items.id, id),
    });
    if (!selectItem) throw new Error("Item ID could not be found");
    await db.delete(items).where(eq(items.id, selectItem.id));

    return new Response("Item deleted", { status: 200 });
  } catch (e) {
    console.log(e);
    return new Response("There was an error deleting the item", {
      status: 400,
    });
  }
};
