import { useMutation } from '@tanstack/react-query';

import { useAccount } from './useAccount';
import { useTransactionClient } from './useTransactionClient';
import { DEFAULT_GAS_COST } from '@/utils/configs';

export function useSendTransaction() {
  const { account } = useAccount();
  const { client } = useTransactionClient();

  const mutation = useMutation({
    mutationFn: async (args: {
      rpc: Buffer<ArrayBufferLike>,
      gasCost?: number
    }) => {
      if (!account) {
        throw new Error("Account not found");
      }

      if (!client) {
        throw new Error("Transaction client not found");
      }

      return await client.signAndSend(
        {
          address: account.address,
          rpc: args.rpc,
        },
        args.gasCost ?? DEFAULT_GAS_COST
      )
    },
  });

  return {
    sendTransaction: mutation.mutate,
    sendTransactionAsync: mutation.mutateAsync,
    ...mutation
  };
}