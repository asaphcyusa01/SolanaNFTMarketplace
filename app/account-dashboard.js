import { useState, useEffect } from 'react';
import {
  BitpieWalletAdapter,
  PhantomWalletAdapter,
} from '@solana/wallet-adapter-bitpie'; 
import './dashboard.css';

const AccountDashboard = () => {
  const [connectedWallet, setConnectedWallet] = useState(null);
  const [nfts, setNFTs] = useState([]);
  const [transactionHistory, setTransactionHistory] = useState([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(null);

  const connectWallet = async (walletAdapter) => {
    try {
      const wallet = new walletAdapter();
      await wallet.connect();
      setConnectedWallet(wallet);
    } catch (error) {
      setError('Error connecting wallet: ' + error.message);
    }
  };

  const fetchNFTs = async () => {
    setLoading(true);
    try {
      if (connectedWallet) {
        
        const sampleNFTs = [
          { name: 'NFT 1', id: 1 },
          { name: 'NFT 2', id: 2 },
          { name: 'NFT 3', id: 3 },
        ];
        setNFTs(sampleNFTs);
      }
    } catch (error) {
      setError('Error fetching NFTs: ' + error.message);
    } finally {
      setLoading(false);
    }
  };

  const fetchTransactionHistory = async () => {
    setLoading(true);
    try {
      if (connectedWallet) {
        // Replace with real data fetching
        const sampleTransactions = [
          { description: 'Transaction 1', id: 1 },
          { description: 'Transaction 2', id: 2 },
          { description: 'Transaction 3', id: 3 },
        ];
        setTransactionHistory(sampleTransactions);
      }
    } catch (error) {
      setError('Error fetching transaction history: ' + error.message);
    } finally {
      setLoading(false);
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
      {error && <div className="error">{error}</div>}
      {loading ? (
        <div>Loading...</div>
      ) : connectedWallet ? (
        <div>
          <h2>Welcome, {connectedWallet.name}!</h2>
          <button onClick={() => connectWallet(BitpieWalletAdapter)}>
            Connect Bitpie Wallet
          </button>
          <button onClick={() => connectWallet(PhantomWalletAdapter)}>
            Connect Phantom Wallet
          </button>
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
          <button onClick={() => connectWallet(BitpieWalletAdapter)}>
            Connect Bitpie Wallet
          </button>
          <button onClick={() => connectWallet(PhantomWalletAdapter)}>
            Connect Phantom Wallet
          </button>
        </div>
      )}
    </div>
  );
};

export default AccountDashboard;
