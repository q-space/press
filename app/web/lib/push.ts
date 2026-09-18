// app/web/lib/push.ts
//
// CHANGELOG:
// - BB26091207: initial web push subscribe/unsubscribe helpers.
/**
 * Web push subscribe/unsubscribe helpers (BB26091207), Android/Chrome.
 * Safari/iOS support for the `Notification`/`PushManager` APIs used here
 * is out of scope -- callers should feature-detect with
 * `isPushSupported()` before showing any opt-in UI.
 */

// --- CONFIG BLOCK ---
const SUBSCRIBE_ENDPOINT = "/api/push/subscribe";
const UNSUBSCRIBE_ENDPOINT = "/api/push/unsubscribe";
// --- END CONFIG BLOCK ---

export function isPushSupported(): boolean {
  return (
    typeof window !== "undefined" &&
    "serviceWorker" in navigator &&
    "PushManager" in window &&
    "Notification" in window
  );
}

/** Web push application server keys are URL-safe base64; PushManager wants a Uint8Array. */
function urlBase64ToUint8Array(base64String: string): Uint8Array {
  const padding = "=".repeat((4 - (base64String.length % 4)) % 4);
  const base64 = (base64String + padding).replace(/-/g, "+").replace(/_/g, "/");
  const rawData = atob(base64);
  const outputArray = new Uint8Array(rawData.length);
  for (let i = 0; i < rawData.length; i += 1) {
    outputArray[i] = rawData.charCodeAt(i);
  }
  return outputArray;
}

export type PushSubscribeResult =
  | { ok: true }
  | { ok: false; reason: "unsupported" | "permission-denied" | "no-vapid-key" | "request-failed" };

/**
 * Requests Notification permission, subscribes via PushManager, and posts
 * the resulting subscription to the Next.js route handler at
 * `/api/push/subscribe`, which forwards it to the Rust API (see
 * `app/api/src/push`). Never calls the Rust API directly from the
 * browser -- same BFF pattern as `lib/api-client.ts`'s `proxyToApi`.
 */
export async function subscribeToPush(handle: string): Promise<PushSubscribeResult> {
  if (!isPushSupported()) return { ok: false, reason: "unsupported" };

  const permission = await Notification.requestPermission();
  if (permission !== "granted") return { ok: false, reason: "permission-denied" };

  const vapidPublicKey = process.env.NEXT_PUBLIC_VAPID_PUBLIC_KEY;
  if (!vapidPublicKey) return { ok: false, reason: "no-vapid-key" };

  const registration = await navigator.serviceWorker.ready;
  const existing = await registration.pushManager.getSubscription();
  const subscription =
    existing ??
    (await registration.pushManager.subscribe({
      userVisibleOnly: true,
      applicationServerKey: urlBase64ToUint8Array(vapidPublicKey),
    }));

  try {
    const res = await fetch(SUBSCRIBE_ENDPOINT, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ handle, subscription: subscription.toJSON() }),
    });
    if (!res.ok) return { ok: false, reason: "request-failed" };
  } catch {
    return { ok: false, reason: "request-failed" };
  }

  return { ok: true };
}

export async function unsubscribeFromPush(handle: string): Promise<void> {
  if (!isPushSupported()) return;
  const registration = await navigator.serviceWorker.ready;
  const subscription = await registration.pushManager.getSubscription();
  if (!subscription) return;

  const endpoint = subscription.endpoint;
  await subscription.unsubscribe();

  try {
    await fetch(UNSUBSCRIBE_ENDPOINT, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ handle, endpoint }),
    });
  } catch {
    // Best-effort: the browser-side unsubscribe already succeeded, which is
    // what stops notifications from showing. A dangling server-side row
    // just means a future send to it 410s -- not a correctness problem.
  }
}
