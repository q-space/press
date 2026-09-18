// app/web/public/sw.js
//
// CHANGELOG:
// - BB26091207: initial reader-scoped service worker (network-first
//   navigation for reader routes, stale-while-revalidate for hashed
//   `_next/static` assets, offline fallback shell, web push handling).
/**
 * Reader PWA service worker (BB26091207).
 *
 * Registered ONLY from `app/(publication)/[handle]/layout.tsx` -- creators
 * (`(creator)`), auth (`(auth)`) and marketing (`(marketing)`) never load
 * the script that calls `navigator.serviceWorker.register()`, so a visitor
 * who never opens a reader page never has this worker installed at all.
 *
 * Registration scope is still site-wide ("/"), because a service worker's
 * effective scope is a property of where the *file* is served from, not of
 * which page called register() -- Chrome would happily let it control
 * `/dashboard` too if we let it. Rather than fight that with a `scope`
 * path (impossible here: `(publication)` is `/[handle]`, a dynamic segment
 * that shares the URL namespace with every other route group, not a fixed
 * prefix), this worker enforces the boundary itself: every fetch handler
 * below inspects the request URL and calls `event.respondWith()` ONLY for
 * requests it recognizes as reader-surface. Everything else falls through
 * untouched -- no interception, no caching, no risk to Creator Studio's
 * client-side rendering or SSR/ISR anywhere else.
 *
 * Canon D-011 constraint this exists to satisfy: readers arrive from a
 * cold WhatsApp-forward link and must see the real, current, crawlable
 * SSR/ISR page -- never a stale cached copy standing in for it. That is
 * why every reader navigation below is NETWORK-FIRST: the network response
 * wins whenever it's reachable, and the cache is purely an offline
 * fallback, never a freshness shortcut. This is also why Googlebot is
 * unaffected: crawlers don't carry a previously-installed service worker
 * across requests, so they always hit the real SSR/ISR response this
 * worker would itself prefer anyway.
 */

// --- CONFIG BLOCK ---
const SW_VERSION = "v1";
const SHELL_CACHE = `qspace-shell-${SW_VERSION}`;
const PAGES_CACHE = `qspace-pages-${SW_VERSION}`;
const ASSET_CACHE = `qspace-assets-${SW_VERSION}`;
const OFFLINE_URL = "/offline";

// Top-level path prefixes that belong to a NON-reader surface (Creator
// Studio, auth, marketing, the app's own API/build/PWA plumbing). Anything
// NOT matching one of these is treated as a reader route
// (`/[handle]` or `/[handle]/[slug]`), per the (publication) route group's
// shape in `app/web/app/(publication)/`. If a new top-level marketing or
// creator route is added outside those groups, add its prefix here too --
// see canon D-011 / the Reader-Studio-Companion split this worker exists
// to respect.
const NON_READER_PREFIXES = [
  "/login",
  "/sign-up",
  "/dashboard",
  "/studio",
  "/editor",
  "/audience",
  "/monetize",
  "/settings",
  "/api/",
  "/_next/",
  "/icons/",
  "/manifest.webmanifest",
  "/sw.js",
  "/offline",
];
// --- END CONFIG BLOCK ---

function isReaderNavigation(url) {
  if (url.origin !== self.location.origin) return false;
  if (url.pathname === "/") return false; // marketing home, (marketing) group
  return !NON_READER_PREFIXES.some((prefix) => url.pathname.startsWith(prefix));
}

self.addEventListener("install", (event) => {
  event.waitUntil(
    caches.open(SHELL_CACHE).then((cache) => cache.addAll([OFFLINE_URL])),
  );
  self.skipWaiting();
});

self.addEventListener("activate", (event) => {
  const keep = new Set([SHELL_CACHE, PAGES_CACHE, ASSET_CACHE]);
  event.waitUntil(
    caches
      .keys()
      .then((names) => Promise.all(names.filter((n) => !keep.has(n)).map((n) => caches.delete(n))))
      .then(() => self.clients.claim()),
  );
});

self.addEventListener("fetch", (event) => {
  const { request } = event;
  if (request.method !== "GET") return; // never intercept mutating requests

  const url = new URL(request.url);

  // Reader page navigations: network-first, cache as an offline fallback
  // only. This is the one path where staleness would be a real SEO/UX
  // regression, so the network is always tried first.
  if (request.mode === "navigate" && isReaderNavigation(url)) {
    event.respondWith(
      fetch(request)
        .then((response) => {
          const copy = response.clone();
          caches.open(PAGES_CACHE).then((cache) => cache.put(request, copy));
          return response;
        })
        .catch(async () => {
          const cached = await caches.match(request);
          if (cached) return cached;
          return caches.match(OFFLINE_URL);
        }),
    );
    return;
  }

  // Next.js build assets are content-hashed and immutable -- safe to
  // stale-while-revalidate. Restricted to requests a reader page actually
  // makes, since non-reader pages never register this worker in the first
  // place, but kept defensive (path-based, not tab-based) in case a
  // reader page is left open in a background tab.
  if (url.origin === self.location.origin && url.pathname.startsWith("/_next/static/")) {
    event.respondWith(
      caches.open(ASSET_CACHE).then(async (cache) => {
        const cached = await cache.match(request);
        const network = fetch(request)
          .then((response) => {
            cache.put(request, response.clone());
            return response;
          })
          .catch(() => cached);
        return cached || network;
      }),
    );
    return;
  }

  // Everything else (API calls, non-reader pages, third-party requests):
  // no respondWith at all -- the browser handles it exactly as it would
  // with no service worker present.
});

// --- Web push (Android/Chrome) ---

self.addEventListener("push", (event) => {
  if (!event.data) return;
  let payload;
  try {
    payload = event.data.json();
  } catch {
    payload = { title: "QSpace Press", body: event.data.text() };
  }
  const title = payload.title || "QSpace Press";
  const options = {
    body: payload.body || "",
    icon: payload.icon || "/icons/icon-192.png",
    badge: payload.badge || "/icons/icon-192.png",
    data: { url: payload.url || "/" },
    tag: payload.tag || undefined,
  };
  event.waitUntil(self.registration.showNotification(title, options));
});

self.addEventListener("notificationclick", (event) => {
  event.notification.close();
  const targetUrl = event.notification.data?.url || "/";
  event.waitUntil(
    self.clients.matchAll({ type: "window", includeUncontrolled: true }).then((clients) => {
      for (const client of clients) {
        if (client.url === targetUrl && "focus" in client) return client.focus();
      }
      if (self.clients.openWindow) return self.clients.openWindow(targetUrl);
    }),
  );
});
