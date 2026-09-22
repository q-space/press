export default function PostPage({ params }: { params: { handle: string; slug: string } }) {
  return (
    <main className="p-8">
      Post {params.slug} in {params.handle} — not yet built (SSR + ISR).
    </main>
  );
}
