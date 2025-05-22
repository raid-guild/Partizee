'use client'

import { createContext, useContext, ReactNode, useState } from 'react';
import PartisiaSdk from 'partisia-sdk';

import { PARTISIA_SDK_CONFIGS } from '@/utils/configs';
import { PermissionTypes } from '@/types/partisia';

interface PartisiaContextType {
  sdk: PartisiaSdk;
  connect: (args?: {
    chainId?: string;
    permissions?: PermissionTypes[];
    dappName?: string;
  }) => Promise<void>;
  isConnected: boolean;
}

const PartisiaContext = createContext<PartisiaContextType | undefined>(undefined);

export function PartisiaProvider({ children }: { children: ReactNode }) {
  const [isConnected, setIsConnected] = useState(false);
  
  const sdk = new PartisiaSdk();

  const connect = async (args?: {
    chainId?: string;
    permissions?: PermissionTypes[];
    dappName?: string;
  }) => {
    try {
      await sdk.connect({
        chainId: args?.chainId || PARTISIA_SDK_CONFIGS.chainId,
        permissions: args?.permissions || PARTISIA_SDK_CONFIGS.permissions,
        dappName: args?.dappName || PARTISIA_SDK_CONFIGS.dappName,
      });
      setIsConnected(true);
    } catch (error) {
      console.error('Failed to connect to Partisia:', error);
      setIsConnected(false);
      throw error;
    }
  };

  return (
    <PartisiaContext.Provider value={{ sdk, connect, isConnected }}>
      {children}
    </PartisiaContext.Provider>
  );
}

export function usePartisia() {
  const context = useContext(PartisiaContext);
  if (context === undefined) {
    throw new Error('usePartisia must be used within a PartisiaProvider');
  }
  return context;
}
