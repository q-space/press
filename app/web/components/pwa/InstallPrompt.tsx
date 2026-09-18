// app/web/components/pwa/InstallPrompt.tsx
//
// CHANGELOG:
// - BB26091207: initial install-prompt banner, deferred beyond first visit
//   per canon D-011.
"use client";

import { useEffect, useState } from "react";

// --- CONFIG BLOCK ---
const VISITS_KEY = "qspace_pwa_visits";
const DISMISSED_KEY = "qspace_pwa_install_dismissed_at";
const DISMISS_COOLDOWN_MS = 14 * 24 * 60 * 60 * 1000; // 14 days
const MIN_VISITS_BEFORE_PROMPT = 2;
// --- END CONFIG BLOCK ---

type BeforeInstallPromptEvent = Event & {
  prompt: () => Promise<void>;
  userChoice: Promise<{ outcome: "accepted" | "dismissed" }>;
};

/**
 * Canon D-011, verbatim: "An install prompt between the link and the
 * content destroys the conversion that is the entire growth engine."
 *
 * Two things follow, both enforced here rather than left to browser
 * defaults:
 *   1. `beforeinstallprompt`'s own automatic mini-infobar is suppressed
 *      (`event.preventDefault()`) on every visit, first one included.
 *   2. This component's own banner does not render on that first visit
 *      either -- it waits for a second reader-route pageview (a crude but
 *      honest proxy for "this reader is coming back", the only signal
 *      available without real analytics wired in yet) before ever
 *      showing anything, and backs off for 14 days once dismissed.
 * The banner itself never blocks the post -- it is a small, dismissible,
 * non-modal bar, and `prompt()` is only ever called from an explicit tap
 * on it, never automatically.
 */
export default function InstallPrompt() {
  const [deferredEvent, setDeferredEvent] = useState<BeforeInstallPromptEvent | null>(null);
  const [visible, setVisible] = useState(false);

  useEffect(() => {
    const isStandalone =
      window.matchMedia?.("(display-mode: standalone)").matches ||
      // iOS Safari's non-standard flag; harmless to read, PWA install
      // itself is Android/Chrome scope per this row.
      (navigator as unknown as { standalone?: boolean }).standalone === true;
    if (isStandalone) return;

    let visits = 0;
    try {
      visits = Number(localStorage.getItem(VISITS_KEY) ?? "0") + 1;
      localStorage.setItem(VISITS_KEY, String(visits));
    } catch {
      // Private browsing / storage blocked -- fall back to "always first visit"
      // so the banner never shows rather than risk showing it too early.
      visits = 1;
    }

    const onBeforeInstallPrompt = (event: Event) => {
      event.preventDefault();
      setDeferredEvent(event as BeforeInstallPromptEvent);

      let dismissedAt = 0;
      try {
        dismissedAt = Number(localStorage.getItem(DISMISSED_KEY) ?? "0");
      } catch {
        /* ignore */
      }
      const recentlyDismissed = dismissedAt > 0 && Date.now() - dismissedAt < DISMISS_COOLDOWN_MS;

      if (visits >= MIN_VISITS_BEFORE_PROMPT && !recentlyDismissed) {
        setVisible(true);
      }
    };

    window.addEventListener("beforeinstallprompt", onBeforeInstallPrompt);
    return () => window.removeEventListener("beforeinstallprompt", onBeforeInstallPrompt);
  }, []);

  if (!visible || !deferredEvent) return null;

  const dismiss = () => {
    setVisible(false);
    try {
      localStorage.setItem(DISMISSED_KEY, String(Date.now()));
    } catch {
      /* ignore */
    }
  };

  const install = async () => {
    setVisible(false);
    await deferredEvent.prompt();
    await deferredEvent.userChoice;
    setDeferredEvent(null);
  };

  return (
    <div
      role="dialog"
      aria-label="Install QSpace Press"
      className="fixed inset-x-4 bottom-4 z-50 mx-auto flex max-w-[420px] items-center gap-3 rounded-xl bg-[#0b0f14] px-4 py-3 text-white shadow-lg"
    >
      <span className="flex-1 text-sm">
        Add QSpace Press to your home screen for faster access.
      </span>
      <button
        onClick={install}
        className="rounded-lg bg-white px-3 py-1.5 text-[13px] font-semibold text-[#0b0f14]"
      >
        Install
      </button>
      <button onClick={dismiss} aria-label="Dismiss" className="text-base leading-none text-white/70">
        ×
      </button>
    </div>
  );
}
