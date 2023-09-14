import { useState, useEffect } from 'react';
import {
  BitpieWalletAdapter,
  PhantomWalletAdapter,
} from '@solana/wallet-adapter-bitpie'; // Replace with your Solana wallet adapters
import './dashboard.css';
const AccountDashboard = () => {
  const [connectedWallet, setConnectedWallet] = useState(null);
  const [nfts, setNFTs] = useState([]);
  const [transactionHistory, setTransactionHistory] = useState([]);

  const connectBitpieWallet = async () => {
    try {
      const wallet = new BitpieWalletAdapter();
      await wallet.connect();
      setConnectedWallet(wallet);
    } catch (error) {
      console.error('Error connecting Bitpie wallet:', error);
    }
  };

  const connectPhantomWallet = async () => {
    try {
      const wallet = new PhantomWalletAdapter();
      await wallet.connect();
      setConnectedWallet(wallet);
    } catch (error) {
      console.error('Error connecting Phantom wallet:', error);
    }
  };

  const fetchNFTs = async () => {
    if (connectedWallet) {
      // Simulate fetching NFTs with placeholder data
      const sampleNFTs = [
        { name: 'NFT 1', id: 1 },
        { name: 'NFT 2', id: 2 },
        { name: 'NFT 3', id: 3 },
      ];
      setNFTs(sampleNFTs);
    }
  };

  const fetchTransactionHistory = async () => {
    if (connectedWallet) {
      // Simulate fetching transaction history with placeholder data
      const sampleTransactions = [
        { description: 'Transaction 1', id: 1 },
        { description: 'Transaction 2', id: 2 },
        { description: 'Transaction 3', id: 3 },
      ];
      setTransactionHistory(sampleTransactions);
    }
  };

  useEffect(() => {
    if (connectedWallet) {
      fetchNFTs();
      fetchTransactionHistory();
    }
  }, [connectedWallet]);

  return (
    <div>
      {connectedWallet ? (
        <div>
          <h2>Welcome, {connectedWallet.name}!</h2>
          <button onClick={connectBitpieWallet}>Connect Bitpie Wallet</button>
          <button onClick={connectPhantomWallet}>Connect Phantom Wallet</button>
          <h3>Your NFTs:</h3>
          <ul>
            {nfts.map((nft) => (
              <li key={nft.id}>{nft.name}</li>
            ))}
          </ul>
          <h3>Transaction History:</h3>
          <ul>
            {transactionHistory.map((transaction) => (
              <li key={transaction.id}>{transaction.description}</li>
            ))}
          </ul>
        </div>
      ) : (
        <div>
          <h2>Connect your wallet</h2>
          <button onClick={connectBitpieWallet}>Connect Bitpie Wallet</button>
          <button onClick={connectPhantomWallet}>Connect Phantom Wallet</button>
        </div>
      )}
    </div>
  );
};

export default AccountDashboard;
