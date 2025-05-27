import { useMutation } from '@tanstack/react-query';

import { usePartisia } from '@/context/partisia';
import { signMessage } from '@/utils/actions';

export function useSignMessage() {
  const { sdk } = usePartisia();

  const mutation = useMutation({
    mutationFn: async (args: {
      contract?: string;
      payload: string;
      payloadType: "utf8" | "hex" | "hex_payload";
      dontBroadcast?: boolean;
    }) => {
      return await signMessage(sdk, args);
    },
  });

  return {
    signMessage: mutation.mutate,
    signMessageAsync: mutation.mutateAsync,
    ...mutation
  };
}
