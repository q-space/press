import { z } from "zod";

/** Mirrors the `subscriptions` table -- canon doc §Data Model. Paid subscription records. */
export const PaymentMethodSchema = z.enum(["mpesa", "stripe"]);
export const SubscriptionStatusSchema = z.enum(["active", "cancelled", "expired", "past_due"]);

export const SubscriptionSchema = z.object({
  id: z.string().uuid(),
  subscriberId: z.string().uuid(),
  publicationId: z.string().uuid(),
  paymentMethod: PaymentMethodSchema,
  status: SubscriptionStatusSchema.default("active"),
  priceKes: z.number().int().nullable(),
  priceUsd: z.number().int().nullable(),
  stripeSubId: z.string().max(100).nullable(),
  currentPeriodStart: z.string().datetime(),
  currentPeriodEnd: z.string().datetime(),
  cancelledAt: z.string().datetime().nullable(),
  createdAt: z.string().datetime(),
  updatedAt: z.string().datetime(),
});

export type Subscription = z.infer<typeof SubscriptionSchema>;
