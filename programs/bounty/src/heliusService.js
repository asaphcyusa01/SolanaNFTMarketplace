import { Helius } from 'helius-sdk';
import { Address, TransactionType } from 'helius-sdk';

const helius = new Helius('68cbe250-e856-4a16-9088-e24b6a80f895');
const solanaRpcUrl = 'https://devnet.helius-rpc.com/?api-key=68cbe250-e856-4a16-9088-e24b6a80f895';

export async function createNFTListingWebhook(webhookURL) {
    try {
        const response = await helius.createWebhook({
            accountAddresses: [Address.MAGIC_EDEN_V2],
            transactionTypes: [TransactionType.NFT_LISTING],
            webhookURL: webhookURL,
        });

        console.log('Helius Webhook created successfully:', response);
    } catch (error) {
        console.error('Error creating Helius webhook:', error);
    }
}

export default helius;
