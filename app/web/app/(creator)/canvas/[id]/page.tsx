"use client";

import { useEffect, useState } from "react";
import CanvasEditor from "@/components/studio/CanvasEditor";
import { CanvasArtifact, getCanvas } from "@/lib/canvas";

/** Canvas's live-editable view (BB26091502) -- what the Studio wizard's
 * "Canvas" action lands the creator on after publish. */
export default function CanvasPage({ params }: { params: { id: string } }) {
  const [canvas, setCanvas] = useState<CanvasArtifact | null>(null);
  const [error, setError] = useState("");

  useEffect(() => {
    let cancelled = false;
    (async () => {
      try {
        const found = await getCanvas(params.id);
        if (!cancelled) setCanvas(found);
      } catch (cause) {
        if (!cancelled) setError(cause instanceof Error ? cause.message : String(cause));
      }
    })();
    return () => {
      cancelled = true;
    };
  }, [params.id]);

  if (error) {
    return <main className="p-8 text-sm text-red-600">Could not load this canvas: {error}</main>;
  }
  if (!canvas) {
    return <main className="p-8 text-sm text-neutral-500">Loading canvas...</main>;
  }
  return <CanvasEditor canvas={canvas} />;
}
