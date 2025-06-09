import { useMutation } from '@tanstack/react-query';
import { SentTransaction } from '@partisiablockchain/blockchain-api-transaction-client';

import { useTransactionClient } from './useTransactionClient';

export function useWaitForTransaction(args?: {
  sentTransaction: SentTransaction
}) {
  const { client } = useTransactionClient();

  const mutation = useMutation({
    mutationFn: async ({
      transaction,
      waitFor = "block-inclusion"
    }: {
      transaction: SentTransaction
      waitFor: "block-inclusion" | "spawned-events"
    }) => {
      if (!client) {
        throw new Error("Transaction client not found");
      }

      if (waitFor === "block-inclusion") {
        return await client.waitForInclusionInBlock(transaction)
      } else if (waitFor === "spawned-events") {
        return await client.waitForSpawnedEvents(transaction)
      } else {
        throw new Error("Invalid wait for type");
      }
    },
  });

  return {
    waitForTransaction: mutation.mutate,
    waitForTransactionAsync: mutation.mutateAsync,
    ...mutation
  };
}