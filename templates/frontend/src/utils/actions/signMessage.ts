import PartisiaSdk from "partisia-sdk";

export async function signMessage(sdk: PartisiaSdk, args: {
  contract?: string;
  payload: string;
  payloadType: "utf8" | "hex" | "hex_payload";
  dontBroadcast?: boolean;
}) {
  return await sdk.signMessage({
    contract: args.contract,
    payload: args.payload,
    payloadType: args.payloadType,
    dontBroadcast: args.dontBroadcast !== undefined ? args.dontBroadcast : true,
  });
}