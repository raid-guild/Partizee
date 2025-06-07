export enum PermissionTypes {
  SIGN = "sign",
  PRIVATE_KEY = "private_key"
}

export interface PartisiaSDKConfigs {
  chainId: "Partisia Blockchain Testnet" | "Partisia Blockchain";
  permissions: PermissionTypes[];
  dappName: string;
}