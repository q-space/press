import { z } from "zod";

/** Mirrors the `subscribers` table -- canon doc §Data Model. */
export const SubscriberTierSchema = z.enum(["free", "paid", "comped", "unsubscribed"]);

export const SubscriberSchema = z.object({
  id: z.string().uuid(),
  publicationId: z.string().uuid(),
  email: z.string().email().max(320),
  name: z.string().max(200).nullable(),
  tier: SubscriberTierSchema.default("free"),
  source: z.string().max(50).nullable(),
  confirmed: z.boolean().default(false),
  confirmToken: z.string().max(64).nullable(),
  subscribedAt: z.string().datetime(),
  confirmedAt: z.string().datetime().nullable(),
  unsubscribedAt: z.string().datetime().nullable(),
});

export type Subscriber = z.infer<typeof SubscriberSchema>;
