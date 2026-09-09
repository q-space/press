import { NextRequest, NextResponse } from "next/server";

/** M-Pesa Daraja callback receiver — not yet built. */
export async function POST(_req: NextRequest) {
  return NextResponse.json({ ok: false, error: "not implemented" }, { status: 501 });
}
