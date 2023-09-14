import { WalletProvider } from '@solana/wallet-adapter-react'; 
import { ConnectionProvider } from '@solana/wallet-adapter-react/node_modules/@solana/wallet-adapter-react'; 
import { BitpieWalletAdapter, PhantomWalletAdapter } from '@solana/wallet-adapter-bitpie';
import Wallet from '../components/Wallet'; 
import 'globals.css';
import 'app.css';
import Head from 'next/head';
import Link from 'next/link';

const yourWallets = [
  {
    name: 'Bitpie',
    url: 'https://www.bitpie.com/',
    adapter: BitpieWalletAdapter,
  },
  {
    name: 'Phantom',
    url: 'https://phantom.app/',
    adapter: PhantomWalletAdapter,
  },
  // Add more wallet configurations as needed
];

const MyApp = ({ Component, pageProps }) => {
  return (
    <ConnectionProvider endpoint="https://devnet.helius-rpc.com/?api-key=68cbe250-e856-4a16-9088-e24b6a80f895">
      <WalletProvider wallets={yourWallets} autoConnect={true} disableDefaultWalletProvider={true}>
        <div>
          <Head>
            <title>NAS ACADEMY MARKETPLACE</title>
            <link rel="icon" href="/favicon.ico" />
          </Head>
          <nav className='border-b p-6' style={{ background: '#03045E' }}>
            <p className='text-4x1 font-bold text-white'>NAS ACADEMY MARKETPLACE</p>
            <div className='flex mt-4 justify-center'>
              <Link href='/'>
                <a className='mr-4'>
                  Main Marketplace
                </a>
              </Link>
              <Link href='/mint-tokens'>
                <a className='mr-4'>
                  Mint Tokens
                </a>
              </Link>
              <Link href='/my-nfts'>
                <a className='mr-4'>
                  My NFTs
                </a>
              </Link>
              <Link href='/account-dashboard'>
                <a className='mr-4'>
                  Account Dashboard
                </a>
              </Link>
              <Wallet />
            </div>
          </nav>
          <Component {...pageProps} />
        </div>
      </WalletProvider>
    </ConnectionProvider>
  );
};

export default MyApp;
