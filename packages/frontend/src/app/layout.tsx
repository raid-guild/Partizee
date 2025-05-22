import type { Metadata } from "next";
import "./globals.css";
import { PartisiaProvider } from "@/context/partisia";

export const metadata: Metadata = {
  title: "Partisia Dapp Template",
  description: "A template for building apps on Partisia Blockchain using the Partisia SDK",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body>
        <PartisiaProvider>
          {children}
        </PartisiaProvider>
      </body>
    </html>
  );
}
