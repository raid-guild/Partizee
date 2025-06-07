import { useMutation } from '@tanstack/react-query';

import { usePartisia } from '@/context/partisia';

export function useRequestPrivateKey() {
  const { sdk } = usePartisia();

  const mutation = useMutation({
    mutationFn: async () => {
      return await sdk.requestKey();
    },
  });

  return {
    requestPrivateKey: mutation.mutate,
    requestPrivateKeyAsync: mutation.mutateAsync,
    ...mutation
  };
}