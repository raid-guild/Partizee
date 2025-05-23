import { useMutation, useQueryClient } from '@tanstack/react-query';
import { usePartisia } from '@/context/partisia';
import { PermissionTypes } from '@/types/partisia';
import { PARTISIA_SDK_CONFIGS } from '@/utils/configs';

export function useConnect() {
  const { sdk } = usePartisia();
  const queryClient = useQueryClient();
  const mutation = useMutation({
    mutationFn: async (args?: {
      chainId?: string;
      permissions?: PermissionTypes[];
      dappName?: string;
    }) => {
      await sdk.connect({
        chainId: args?.chainId || PARTISIA_SDK_CONFIGS.chainId,
        permissions: args?.permissions || PARTISIA_SDK_CONFIGS.permissions,
        dappName: args?.dappName || PARTISIA_SDK_CONFIGS.dappName,
      });
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['account'] });
    },
  });

  return {
    connect: (args?: {
      chainId?: string;
      permissions?: PermissionTypes[];
      dappName?: string;
    }) => mutation.mutate(args),
    connectAsync: (args?: {
      chainId?: string;
      permissions?: PermissionTypes[];
      dappName?: string;
    }) => mutation.mutateAsync(args),
    ...mutation
  };
}
