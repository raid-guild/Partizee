# Partisia Next.js Boilerplate App

This is a Next.js template for building apps on Partisia Blockchain using the Partisia SDK. It includes some basic actions to help you get started with your app development.

## Configuration

### Partisia Blockchain Settings
Navigate to `src/utils/configs.ts` to configure your application:

- `PARTISIA_SDK_CONFIGS` for your dapp name, wallet permissions and chain to connect with user wallets.
- `CONTRACT_ADDRESS` of your deployed smart contract
- `TESTNET_URL` for RPC connection to Partisia Blockchain
- `DEFAULT_GAS_COST` for transactions (based on your requirements)

### ABI Generation
To generate the ABI of your contract in TypeScript for interacting in your app:

```bash
npm run codegen-abi
# or
yarn codegen-abi
# or
pnpm codegen-abi
# or
bun codegen-abi
```

This will generate the ABI of `CONTRACT_ADDRESS` set in your `/src/utils/configs.ts` and output to `/src/utils/abi.ts`.

## Available Hooks

The template provides several React hooks for interacting with Partisia Blockchain:

- `useAccount`: Get the connected account information
- `useConnect`: Connect to Partisia wallet
- `useSignMessage`: Sign messages using the connected wallet
- `useWriteContract`: Write to smart contracts
- `useRequestPrivateKey`: Request private key access
- `useSendTransaction`: Send transactions
- `useTransactionClient`: Get transaction client instance
- `useWaitForTransaction`: Wait for transaction confirmation

## Getting Started

First, run the development server:

```bash
npm run dev
# or
yarn dev
# or
pnpm dev
# or
bun dev
```

Open [http://localhost:3000](http://localhost:3000) with your browser to see the result.

You can start editing the page by modifying `app/page.tsx`. The page auto-updates as you edit the file.

This project uses [`next/font`](https://nextjs.org/docs/app/building-your-application/optimizing/fonts) to automatically optimize and load [Geist](https://vercel.com/font), a new font family for Vercel.

## Learn More

To learn more about Next.js, take a look at the following resources:

- [Next.js Documentation](https://nextjs.org/docs) - learn about Next.js features and API.
- [Learn Next.js](https://nextjs.org/learn) - an interactive Next.js tutorial.

You can check out [the Next.js GitHub repository](https://github.com/vercel/next.js) - your feedback and contributions are welcome!

## Deploy on Vercel

The easiest way to deploy your Next.js app is to use the [Vercel Platform](https://vercel.com/new?utm_medium=default-template&filter=next.js&utm_source=create-next-app&utm_campaign=create-next-app-readme) from the creators of Next.js.

Check out our [Next.js deployment documentation](https://nextjs.org/docs/app/building-your-application/deploying) for more details.
