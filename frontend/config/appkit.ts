import { createAppKit } from '@reown/appkit'
import { EthersAdapter } from '@reown/appkit-adapter-ethers'
import { mainnet, sepolia } from '@reown/appkit/networks'

// Get projectId from environment variable
const projectId = process.env.NEXT_PUBLIC_PROJECT_ID || ''

if (!projectId) {
  console.warn('NEXT_PUBLIC_PROJECT_ID is not set. Please set it in your .env.local file')
}

// Create the AppKit instance
export const appKit = createAppKit({
  adapters: [new EthersAdapter()],
  networks: [mainnet, sepolia],
  projectId,
  metadata: {
    name: 'OnClick',
    description: 'Decentralized platform for creators, businesses, and crowdfunders',
    url: typeof window !== 'undefined' ? window.location.origin : 'http://localhost:3000',
    icons: ['https://avatars.githubusercontent.com/u/179229932']
  }
})
