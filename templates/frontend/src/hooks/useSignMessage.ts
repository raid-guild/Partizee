import { useMutation } from '@tanstack/react-query';
import { usePartisia } from '@/context/partisia';

export function useSignMessage() {
  const { sdk } = usePartisia();

  const mutation = useMutation({
    mutationFn: async (args: {
      contract?: string;
      payload: string;
      payloadType: "utf8" | "hex" | "hex_payload";
      dontBroadcast?: boolean;
    }) => {
      await sdk.signMessage({
        contract: args.contract,
        payload: args.payload,
        payloadType: args.payloadType,
        dontBroadcast: args.dontBroadcast,
      });
    },
  });

  return {
    signMessage: mutation.mutate,
    signMessageAsync: mutation.mutateAsync,
    ...mutation
  };
}
