import { useQuery } from '@tanstack/react-query';
import { usePartisia } from '@/context/partisia';

export function useAccount() {
  const { sdk } = usePartisia();

  const query = useQuery({
    queryKey: ['account'],
    queryFn: async () => {
      return {
        connection: sdk.connection,
        seed: sdk.seed,
        isConnected: sdk.isConnected,
      }
    },
  });

  return {
    connection: query.data?.connection,
    seed: query.data?.seed,
    isConnected: query.data?.isConnected,
    ...query,
  };
}
