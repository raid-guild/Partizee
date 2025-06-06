'use client'

import { createContext, useContext, ReactNode } from 'react';
import PartisiaSdk from 'partisia-sdk';

interface PartisiaContextType {
  sdk: PartisiaSdk;
}

const PartisiaContext = createContext<PartisiaContextType | undefined>(undefined);

export function PartisiaProvider({
  children,
  sdk,
}: {
  children: ReactNode;
  sdk: PartisiaSdk;
}) {
  return (
    <PartisiaContext.Provider value={{ sdk }}>
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
