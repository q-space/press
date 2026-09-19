import StudioWizard from "@/components/studio/StudioWizard";

export const metadata = {
  title: "Creator Studio — QSpace Press",
};

/**
 * Creator Studio: the structured-document surface. Freeform posts go
 * through the Tiptap editor instead; this is for documents whose shape an
 * archetype declares.
 */
export default function StudioPage() {
  return (
    <main className="min-h-screen bg-neutral-50">
      <StudioWizard />
    </main>
  );
}
