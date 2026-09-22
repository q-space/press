import type { Metadata } from "next";
import "../styles/globals.css";

export const metadata: Metadata = {
  title: "QSpace Press",
  description: "Publish. Reach. Earn. Built for the African professional voice.",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  );
}
