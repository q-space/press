import NextAuth from "next-auth";
import Google from "next-auth/providers/google";

/**
 * Session + token management. Google OAuth only, per the canon doc's own
 * MVP scope. GOOGLE_CLIENT_ID/GOOGLE_CLIENT_SECRET are read from env at
 * runtime -- not yet provisioned (BB26090904/Pre-Cycle 0 signups), so this
 * boots with no providers configured until they exist. AUTH_SECRET is
 * required by next-auth v5 in production; set it alongside the OAuth
 * credentials when they're provisioned.
 */
export const { handlers, auth, signIn, signOut } = NextAuth({
  providers: process.env.GOOGLE_CLIENT_ID
    ? [
        Google({
          clientId: process.env.GOOGLE_CLIENT_ID,
          clientSecret: process.env.GOOGLE_CLIENT_SECRET,
        }),
      ]
    : [],
});
