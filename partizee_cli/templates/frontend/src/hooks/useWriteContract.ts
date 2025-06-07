import { useMutation } from '@tanstack/react-query';
import { usePartisia } from '@/context/partisia';

export function useWriteContract() {
  const { sdk } = usePartisia();

  const mutation = useMutation({
    mutationFn: async (args: {
      contract: string;
      payload: string;
      payloadType?: "utf8" | "hex" | "hex_payload";
      dontBroadcast?: boolean;
    }) => {
      await sdk.signMessage({
        contract: args.contract,
        payload: args.payload,
        payloadType: args.payloadType ?? "hex_payload",
        dontBroadcast: args.dontBroadcast !== undefined ? args.dontBroadcast : false,
      });
    },
  });

  return {
    writeContract: mutation.mutate,
    writeContractAsync: mutation.mutateAsync,
    ...mutation
  };
}