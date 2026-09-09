export default function PublicationHome({ params }: { params: { handle: string } }) {
  return <main className="p-8">Publication {params.handle} — post list, not yet built (SSR).</main>;
}
