"use client"

import { PartisiaProvider } from "@/context/partisia";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import PartisiaSdk from "partisia-sdk";

declare global {
  interface Window {
    ethereum?: {
      request: (args: { method: string; params?: any }) => Promise<any>;
    };
  }
}

const queryClient = new QueryClient();
const sdk = new PartisiaSdk();

export function Providers({ children }: { children: React.ReactNode }) {
  return (
    <QueryClientProvider client={queryClient}>
      <PartisiaProvider sdk={sdk}>
        {children}
      </PartisiaProvider>
    </QueryClientProvider>
  );
}