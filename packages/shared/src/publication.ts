import { z } from "zod";

/** Mirrors the `publications` table -- canon doc §Data Model. One per creator (MVP), many post-launch. */
export const PublicationSchema = z.object({
  id: z.string().uuid(),
  userId: z.string().uuid(),
  handle: z.string().max(30),
  displayName: z.string().max(200),
  tagline: z.string().max(300).nullable(),
  aboutHtml: z.string().nullable(),
  avatarUrl: z.string().max(500).nullable(),
  coverImageUrl: z.string().max(500).nullable(),
  category: z.string().max(50).nullable(),
  customDomain: z.string().max(255).nullable(),
  customDomainVerified: z.boolean().default(false),
  twitterHandle: z.string().max(50).nullable(),
  linkedinUrl: z.string().max(300).nullable(),
  timezone: z.string().max(50).default("Africa/Nairobi"),
  isMonetized: z.boolean().default(false),
  monthlyPriceKes: z.number().int().nullable(),
  monthlyPriceUsd: z.number().int().nullable(),
  stripeAccountId: z.string().max(100).nullable(),
  mpesaPaybill: z.string().max(20).nullable(),
  createdAt: z.string().datetime(),
  updatedAt: z.string().datetime(),
});

export type Publication = z.infer<typeof PublicationSchema>;
