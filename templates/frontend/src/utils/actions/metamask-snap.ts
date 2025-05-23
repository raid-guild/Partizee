import { MetaMaskSnapMethod, MetaMaskSnapRequestMethod } from "@/types/metamask-snap";

const SNAP_ID = 'npm:@partisiablockchain/snap';

export async function requestMetaMaskSnap(
  method: MetaMaskSnapMethod,
  params?: {
    method: MetaMaskSnapRequestMethod;
    params?: any;
  },
) {
  if (!window.ethereum) {
    throw new Error('MetaMask not found');
  }

  if (method === 'wallet_requestSnaps') {
    return window.ethereum?.request({
      method: 'wallet_requestSnaps',
      params: {
        [SNAP_ID]: {},
      },
    });
  }

  if (!params?.method) {
    throw new Error('method is required for wallet_invokeSnap');
  }

  return window.ethereum?.request({
    method,
    params: {
      snapId: SNAP_ID,
      request: {
        method: params?.method,
        params: params?.params,
      },
    },
  })
}