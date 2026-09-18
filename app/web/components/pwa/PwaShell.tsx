// app/web/components/pwa/PwaShell.tsx
//
// CHANGELOG:
// - BB26091207: initial reader-scoped PWA shell (SW registration + install
//   prompt + push opt-in).
"use client";

import { useEffect } from "react";
import InstallPrompt from "./InstallPrompt";
import PushOptIn from "./PushOptIn";

// --- CONFIG BLOCK ---
const SERVICE_WORKER_URL = "/sw.js";
const SERVICE_WORKER_SCOPE = "/";
// --- END CONFIG BLOCK ---

/**
 * Mounted only from `app/(publication)/[handle]/layout.tsx` (BB26091207)
 * -- this is the one place in the app that ever calls
 * `serviceWorker.register()`, so Creator Studio/auth/marketing visitors
 * never install the worker at all. See `public/sw.js`'s own doc comment
 * for the second layer of scoping (path-filtered fetch handling), which
 * covers the case where a reader-route worker is still controlling a tab
 * that later navigates to a non-reader route client-side.
 */
export default function PwaShell({ handle }: { handle: string }) {
  useEffect(() => {
    if (!("serviceWorker" in navigator)) return;
    navigator.serviceWorker.register(SERVICE_WORKER_URL, { scope: SERVICE_WORKER_SCOPE }).catch((err) => {
      console.error("QSpace Press: service worker registration failed", err);
    });
  }, []);

  return (
    <>
      <InstallPrompt />
      <PushOptIn handle={handle} />
    </>
  );
}
