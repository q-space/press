// app/web/components/pwa/PushOptIn.tsx
//
// CHANGELOG:
// - BB26091207: initial explicit push opt-in button.
"use client";

import { useEffect, useState } from "react";
import { isPushSupported, subscribeToPush, unsubscribeFromPush } from "@/lib/push";

/**
 * Explicit, opt-in push button for a publication's reader route
 * (BB26091207). Same non-intrusive posture as `InstallPrompt`: this never
 * triggers itself -- a reader taps it, nothing requests Notification
 * permission on page load.
 */
export default function PushOptIn({ handle }: { handle: string }) {
  const [supported, setSupported] = useState(false);
  const [subscribed, setSubscribed] = useState(false);
  const [status, setStatus] = useState<string | null>(null);

  useEffect(() => {
    setSupported(isPushSupported());
    if (!isPushSupported()) return;
    navigator.serviceWorker.ready
      .then((reg) => reg.pushManager.getSubscription())
      .then((sub) => setSubscribed(Boolean(sub)))
      .catch(() => {});
  }, []);

  if (!supported) return null;

  const toggle = async () => {
    setStatus(null);
    if (subscribed) {
      await unsubscribeFromPush(handle);
      setSubscribed(false);
      return;
    }
    const result = await subscribeToPush(handle);
    if (result.ok) {
      setSubscribed(true);
    } else {
      setStatus(
        result.reason === "permission-denied"
          ? "Notifications are blocked in your browser settings."
          : result.reason === "no-vapid-key"
            ? "Push isn't configured yet on this deployment."
            : "Couldn't enable notifications -- try again in a moment.",
      );
    }
  };

  return (
    <div className="text-[13px]">
      <button onClick={toggle} className="rounded-full border border-current px-3 py-1.5">
        {subscribed ? "Notifications on" : "Get notified of new posts"}
      </button>
      {status && (
        <p role="status" className="mt-1 opacity-75">
          {status}
        </p>
      )}
    </div>
  );
}
