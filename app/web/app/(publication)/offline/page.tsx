// app/web/app/(publication)/offline/page.tsx
//
// CHANGELOG:
// - BB26091207: initial offline fallback shell.
/**
 * Offline fallback shell (BB26091207). Precached by `public/sw.js` at
 * install time and served, network-first-with-fallback, whenever a
 * previously-visited reader page can't reach the network. Deliberately
 * static -- no data fetching, so it always renders even with zero
 * connectivity.
 */
export const dynamic = "force-static";

export default function OfflinePage() {
  return (
    <main className="mx-auto max-w-[480px] p-8 text-center">
      <h1 className="mb-2 text-2xl">You&rsquo;re offline</h1>
      <p className="opacity-75">
        This post hasn&rsquo;t been saved for offline reading yet. Posts you&rsquo;ve
        already opened will still load here without a connection.
      </p>
      <p className="mt-6">
        <a href="/" className="underline">
          Try again
        </a>
      </p>
    </main>
  );
}
