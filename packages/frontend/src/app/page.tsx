"use client"

import { usePartisia } from "@/context/partisia";
import { useEffect } from "react";

export default function Home() {
  const { sdk, connect, isConnected } = usePartisia();

  useEffect(() => {
    if (isConnected) {
      console.log("Connected to Partisia");
    }
  }, [isConnected]);
  
  return (
    <div className="flex flex-col items-center justify-center h-screen">
      <h1 className="text-2xl font-bold">Hello Partisia Blockchain</h1>
      <p className="text-sm text-gray-500">
        This is a Next.js template for building apps on Partisia Blockchain using the Partisia SDK.
      </p>
      <p className="text-sm text-gray-500">
        It includes some basic actions to help you get started. Try them out below.
      </p>
      <div className="flex flex-col gap-2 mt-4">
        <button className="bg-blue-500 text-white p-2 rounded-md" onClick={() => connect()}>Connect Wallet</button>
      </div>
    </div>
  );
}
