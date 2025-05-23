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
      <h1 className="text-2xl font-bold mb-4">👋 Hello Partisia Blockchain</h1>
      <p>
        This is a Next.js template for building apps on Partisia Blockchain using the Partisia SDK.
      </p>
      <p>
        It includes some basic actions to help you get started. Try them out below.
      </p>
      <div className="flex flex-col gap-4 mt-8">
        <button className="bg-blue-500 text-white p-2 rounded-md" onClick={() => connect()}>Connect Wallet</button>
      </div>
      <div className="flex flex-col gap-2 mt-12 text-sm justify-center items-center">
        <p>Don't have a Partisia wallet? Download one:</p>
        <p><a className="text-blue-500" href="https://snaps.metamask.io/snap/npm/partisiablockchain/snap/" target="_blank">MetaMask Snap</a> | <a className="text-blue-500" href="https://chromewebstore.google.com/detail/parti-wallet/gjkdbeaiifkpoencioahhcilildpjhgh" target="_blank">Parti Wallet</a></p>
        <p> Check out the ecosystem of wallets <a className="text-blue-500" href="https://partisiablockchain.com/develop/ecosystem/?_ecosystem=wallets" target="_blank">here</a></p>
      </div>
    </div>
  );
}
