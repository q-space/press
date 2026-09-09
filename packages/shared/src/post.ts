import { z } from "zod";

/** Mirrors the `posts` table -- canon doc §Data Model. */
export const PostStatusSchema = z.enum(["draft", "published", "scheduled"]);

export const PostSchema = z.object({
  id: z.string().uuid(),
  publicationId: z.string().uuid(),
  title: z.string().max(200),
  slug: z.string().max(250),
  contentHtml: z.string(),
  excerpt: z.string().max(500).nullable(),
  coverImageUrl: z.string().max(500).nullable(),
  seoTitle: z.string().max(200).nullable(),
  seoDescription: z.string().max(300).nullable(),
  isPaidOnly: z.boolean().default(false),
  sendNewsletter: z.boolean().default(true),
  status: PostStatusSchema.default("draft"),
  publishedAt: z.string().datetime().nullable(),
  scheduledFor: z.string().datetime().nullable(),
  newsletterSent: z.boolean().default(false),
  newsletterSentAt: z.string().datetime().nullable(),
  createdAt: z.string().datetime(),
  updatedAt: z.string().datetime(),
});

export type Post = z.infer<typeof PostSchema>;
