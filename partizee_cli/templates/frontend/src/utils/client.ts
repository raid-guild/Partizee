import {
  BlockchainTransactionClient,
  ChainControllerApi,
  Configuration,
  SenderAuthentication,
} from "@partisiablockchain/blockchain-api-transaction-client";

import { TESTNET_URL } from "./configs";

export const CLIENT = new ChainControllerApi(
  new Configuration({ basePath: TESTNET_URL })
);

export function getTransactionClient(account: SenderAuthentication) {
  return BlockchainTransactionClient.create(TESTNET_URL, account);
}