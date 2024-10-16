import { z } from "astro:content";
import { defineCollection } from "astro:content";

const blog = defineCollection({
  type: "content",
  // Feel free to move schema definition to separate files if there are too many to define
  schema: z.object({
    title: z.string().max(60),
    description: z.string().max(300),
    pubDate: z.date(),
  }),
});

export const collections = { blog };
