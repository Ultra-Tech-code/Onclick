'use client';

import { createContext, useContext, useState, useEffect } from 'react';
import { AppKitProvider } from '@reown/appkit/react';
import { EthersAdapter } from '@reown/appkit-adapter-ethers';
import { mainnet, sepolia } from '@reown/appkit/networks';

interface ThemeContextType {
  isDark: boolean;
  toggleTheme: () => void;
}

const ThemeContext = createContext<ThemeContextType | undefined>(undefined);

export function useTheme() {
  const context = useContext(ThemeContext);
  if (context === undefined) {
    throw new Error('useTheme must be used within a ThemeProvider');
  }
  return context;
}

const projectId = process.env.NEXT_PUBLIC_PROJECT_ID || '';

export function Providers({ children }: { children: React.ReactNode }) {
  const [isDark, setIsDark] = useState(false);

  useEffect(() => {
    const savedTheme = localStorage.getItem('theme');
    const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
    setIsDark(savedTheme === 'dark' || (!savedTheme && prefersDark));
  }, []);

  useEffect(() => {
    localStorage.setItem('theme', isDark ? 'dark' : 'light');
    if (isDark) {
      document.documentElement.classList.add('dark');
    } else {
      document.documentElement.classList.remove('dark');
    }
  }, [isDark]);

  const toggleTheme = () => {
    setIsDark(!isDark);
  };

  if (!projectId) {
    console.warn('NEXT_PUBLIC_PROJECT_ID is not set');
  }

  return (
    <AppKitProvider
      adapters={[new EthersAdapter()]}
      networks={[mainnet, sepolia]}
      projectId={projectId}
      metadata={{
        name: 'OnClick',
        description: 'Decentralized platform for creators, businesses, and crowdfunders',
        url: typeof window !== 'undefined' ? window.location.origin : 'http://localhost:3000',
        icons: ['https://avatars.githubusercontent.com/u/179229932']
      }}
    >
      <ThemeContext.Provider value={{ isDark, toggleTheme }}>
        {children}
      </ThemeContext.Provider>
    </AppKitProvider>
  );
}
