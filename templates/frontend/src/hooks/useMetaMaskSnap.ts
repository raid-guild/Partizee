import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { PermissionTypes } from '@/types/partisia';
import { requestMetaMaskSnap } from '@/utils/actions/metamask-snap';


export function useRequestMetaMaskSnap() {
  const queryClient = useQueryClient();
  
  const mutation = useMutation({
    mutationFn: async () => {
      return await requestMetaMaskSnap('wallet_requestSnaps');
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['metamask_snap_address'] });
    },
  });

  return {
    connect: () => mutation.mutate(),
    connectAsync: () => mutation.mutateAsync(),
    ...mutation
  };
}


export function useMetaMaskSnapAddress() {
  const mutation = useMutation({
    mutationFn: async () => {
      return await requestMetaMaskSnap('wallet_invokeSnap', {
        method: 'get_address',
      })
    },
  });

  return {
    getAddress: mutation.mutate,
    getAddressAsync: mutation.mutateAsync,
    ...mutation,
  };
}


export function useMetaMaskSnapSignTransaction() {
  const mutation = useMutation({
    mutationFn: async (args: {
      payload: string;
      chainId: string;
    }) => {
      return await requestMetaMaskSnap('wallet_invokeSnap', {
        method: 'sign_transaction',
        params: {
          payload: args.payload,
          chainId: args.chainId,
        },
      });
    },
  });

  return {
    signTransaction: mutation.mutate,
    signTransactionAsync: mutation.mutateAsync,
    ...mutation
  };
}