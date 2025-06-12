import { PartisiaSDKConfigs, PermissionTypes } from "@/types/partisia";

// Edit these configs to your own needs

// Partisia SDK Configs
export const PARTISIA_SDK_CONFIGS: PartisiaSDKConfigs = {
  chainId: "Partisia Blockchain Testnet",
  permissions: [PermissionTypes.SIGN],
  dappName: "Partisia Dapp Template",
};

// Contract Configs
export const CONTRACT_ADDRESS = "02acd7b6a57682122078bf47b8a3fafeca38ddf5a1"

export const TESTNET_URL = "https://node1.testnet.partisiablockchain.com"

export const DEFAULT_GAS_COST = 100_000;