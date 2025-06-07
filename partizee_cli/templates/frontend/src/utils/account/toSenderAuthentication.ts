import PartisiaSdk from "partisia-sdk";
import { SenderAuthentication } from "@partisiablockchain/blockchain-api-transaction-client";

import { signMessage } from "../actions/signMessage";

export function toSenderAuthentication(sdk: PartisiaSdk): SenderAuthentication {
  if (!sdk.isConnected) {
    throw new Error("SDK is not connected");
  }

  const address = sdk.connection?.account?.address;

  return {
    getAddress: () => address!,
    sign: async (transactionPayload: Buffer) => {
      const res = await signMessage(sdk, {
        payload: transactionPayload.toString("hex"),
        payloadType: "hex_payload",
        dontBroadcast: false,
      });
      return res.signature;
    },
  }
}