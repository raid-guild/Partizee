"use client"

import { useEffect } from "react";
import { partisiaCrypto } from "partisia-crypto";
import { useConnect, useAccount, useSignMessage, useWriteContract } from "@/hooks";
import { CONTRACT_ADDRESS } from "@/utils/configs";

export default function Home() {
  const { account, isConnected } = useAccount();
  const { connect, error: connectError } = useConnect();
  const { signMessage, error: signMessageError } = useSignMessage();
  const { writeContract, error: writeContractError } = useWriteContract();

  useEffect(() => {
    if (isConnected) {
      console.log("Connected to Partisia");
    }
    if (connectError) {
      console.error(connectError);
    }
    if (signMessageError) {
      console.error(signMessageError);
    }
    if (writeContractError) {
      console.error(writeContractError);
    }
  }, [isConnected, connectError, signMessageError, writeContractError]);

  const abi = partisiaCrypto.abi_system.Payload_ContractAbi
  // const abi = partisiaCrypto.transaction.getContractAbi(CONTRACT_ADDRESS, 0)
  
  const payload = partisiaCrypto.structs.serializeToBuffer({
    counter: 0,
    increment_amount: 5,
  }, ...abi).toString("hex")
  
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
        <div className="flex flex-col gap-2 justify-center items-center">
          <p>Account:</p>
          <p>{account?.address ? "✅ Connected" : "❌ Not connected"}</p>
        </div>
        <button 
          className="bg-blue-500 text-white p-2 rounded-md disabled:opacity-50 cursor-pointer disabled:cursor-not-allowed" 
          onClick={() => connect()}
          disabled={isConnected}
        >
          Connect Wallet
        </button>
        <button 
          className="bg-blue-500 text-white p-2 rounded-md disabled:opacity-50 cursor-pointer disabled:cursor-not-allowed" 
          onClick={() => signMessage({
            payload: "Hello Partisia",
            payloadType: "utf8",
          })}
          disabled={!isConnected}
        >
          Sign Message
        </button>
        <button 
          className="bg-blue-500 text-white p-2 rounded-md disabled:opacity-50 cursor-pointer disabled:cursor-not-allowed" 
          onClick={() => writeContract({
            contract: CONTRACT_ADDRESS,
            payload,
          })}
          disabled={!isConnected}
        >
          Write Contract
        </button>
      </div>
      <div className="flex flex-col gap-2 mt-12 text-sm justify-center items-center">
        <p>Don't have a Partisia wallet? Download one:</p>
        <p><a className="text-blue-500" href="https://snaps.metamask.io/snap/npm/partisiablockchain/snap/" target="_blank">MetaMask Snap</a> | <a className="text-blue-500" href="https://chromewebstore.google.com/detail/parti-wallet/gjkdbeaiifkpoencioahhcilildpjhgh" target="_blank">Parti Wallet</a></p>
        <p> Check out the ecosystem of wallets <a className="text-blue-500" href="https://partisiablockchain.com/develop/ecosystem/?_ecosystem=wallets" target="_blank">here</a></p>
      </div>
    </div>
  );
}
