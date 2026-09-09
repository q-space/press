import { z } from "zod";
import { PaymentMethodSchema } from "./subscription";

/** Mirrors the `payments` table -- canon doc §Data Model. Individual payment transactions. */
export const PaymentStatusSchema = z.enum(["pending", "confirmed", "failed", "refunded"]);

export const PaymentSchema = z.object({
  id: z.string().uuid(),
  subscriptionId: z.string().uuid().nullable(),
  subscriberId: z.string().uuid(),
  publicationId: z.string().uuid(),
  paymentMethod: PaymentMethodSchema,
  amountKes: z.number().int().nullable(),
  amountUsd: z.number().int().nullable(),
  platformFeeKes: z.number().int().nullable(),
  netToCreatorKes: z.number().int().nullable(),
  status: PaymentStatusSchema.default("pending"),
  mpesaCheckoutRequestId: z.string().max(100).nullable(),
  mpesaReceiptNumber: z.string().max(50).nullable(),
  stripePaymentIntentId: z.string().max(100).nullable(),
  errorMessage: z.string().nullable(),
  createdAt: z.string().datetime(),
  confirmedAt: z.string().datetime().nullable(),
});

export type Payment = z.infer<typeof PaymentSchema>;
