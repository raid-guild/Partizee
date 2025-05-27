import { useQuery } from '@tanstack/react-query';

import { usePartisia } from '@/context/partisia';
import { useAccount } from './useAccount';
import { toSenderAuthentication } from '@/utils/account';
import { getTransactionClient } from '@/utils/client';

export function useTransactionClient() {
  const { sdk } = usePartisia();
  const { account, isConnected } = useAccount();

  const query = useQuery({
    queryKey: ["transactionClient", account?.address],
    queryFn: () => {
      if (!isConnected) {
        throw new Error("Account not connected");
      }

      return getTransactionClient(toSenderAuthentication(sdk));
    },
    enabled: isConnected,
  });

  return {
    client: query.data,
    ...query,
  };
}
