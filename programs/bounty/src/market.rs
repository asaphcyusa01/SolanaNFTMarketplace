use anchor_lang::prelude::*;
use anchor_spl::token::Token;
use anchor_spl::token_metadata::TokenMetadata;
use anchor_spl::token::Mint;
use anchor_spl::token::Transfer;
use anchor_lang::solana_program::pubkey::Pubkey;
use anchor_lang::solana_program::sysvar::clock::Clock;
use anchor_lang::solana_program::entrypoint::ProgramResult;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod nft_marketplace {
    use super::*;
    
    pub fn create_market_item(ctx: Context<CreateMarketItem>, price: u64) -> ProgramResult {
        let mint_authority = &ctx.accounts.mint_authority;

        // Create a new MarketToken
        let token_metadata = TokenMetadataV2 {
            data: Data {
                name: Some("MoonBirdz".to_string()),
                symbol: Some("MBIRDZ".to_string()),
                uri: "".to_string(), // You'll need to set the URI.
                seller_fee_basis_points: 500, // Example seller fee.
                creators: Some(vec![Creator {
                    address: ctx.accounts.creator.key().clone(),
                    verified: true,
                    share: 10000, // 100% share to the creator.
                }]),
            },
            primary_sale_happened: false,
            is_mutable: true,
            mint: *ctx.accounts.mint.to_account_info().key,
            update_authority: *ctx.accounts.update_authority.to_account_info().key,
            token: *ctx.accounts.nft.to_account_info().key,
            seller_fee_collection_token: *ctx.accounts.seller_fee.to_account_info().key,
            creators_configured: true,
        };

        // Transfer the NFT to the marketplace
        let cpi_accounts = Transfer {
            from: ctx.accounts.mint_authority.to_account_info(),
            to: ctx.accounts.nft.to_account_info(),
            authority: ctx.accounts.nft_authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.clone();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, 1)?;

        // Mint the MarketToken
        let cpi_accounts = CreateMetadata {
            metadata: ctx.accounts.nft_metadata.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            mint_authority: ctx.accounts.mint_authority.to_account_info(),
            payer: ctx.accounts.payer.to_account_info(),
            update_authority: ctx.accounts.update_authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_metadata_program.clone();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token_metadata::create_metadata(cpi_ctx, token_metadata)?;

        // Create the MarketToken
        let token_id = ctx.accounts.nft_metadata.data.token_id;
        let market_token = MarketToken {
            item_id: token_id,
            nft_contract: *ctx.accounts.nft.to_account_info().key,
            token_id: token_id,
            seller: *ctx.accounts.creator.to_account_info().key,
            owner: Pubkey::new(&[0; 32]), // Set to the initial owner's pubkey.
            price: price,
            sold: false,
        };

        // Store the MarketToken
        let index = next_market_token_id(ctx.accounts.nft_marketplace.to_account_info())?;
        let mut market_tokens = MarketTokens::from(&mut ctx.accounts.nft_marketplace_market_tokens);
        market_tokens.push(&market_token)?;
        market_tokens.serialize(&mut &mut ctx.accounts.nft_marketplace_market_tokens.data.borrow_mut()[..])?;

        // Increment the next market token ID
        ctx.accounts.nft_marketplace_market_tokens.data.borrow_mut()[8..16].copy_from_slice(&index.to_le_bytes());

        Ok(())
    }
    
    pub fn buy_market_item(ctx: Context<BuyMarketItem>, item_id: u64) -> ProgramResult {
        // Find the market token by item_id.
        let index = next_market_token_id(ctx.accounts.nft_marketplace.to_account_info())?;
        let market_token_index = index.checked_sub(1).ok_or(ErrorCode::InvalidArgument)?;
        let mut market_tokens = MarketTokens::from(&mut ctx.accounts.nft_marketplace_market_tokens)?;
        let market_token = market_tokens
            .get(market_token_index as usize)
            .ok_or(ErrorCode::InvalidArgument)?;
    
        // Ensure the item is not already sold.
        if market_token.sold {
            return Err(ErrorCode::InvalidArgument.into());
        }
    
        // Transfer funds from the buyer to the seller.
        let cpi_accounts = Transfer {
            from: ctx.accounts.buyer.to_account_info(),
            to: ctx.accounts.seller.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.clone();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, market_token.price)?;
    
        // Transfer the ownership of the NFT.
        let cpi_accounts = Transfer {
            from: ctx.accounts.nft.to_account_info(),
            to: ctx.accounts.buyer.to_account_info(),
            authority: ctx.accounts.nft_authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.clone();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, 1)?;
    
        // Mark the item as sold.
        market_tokens[market_token_index as usize].sold = true;
        market_tokens.serialize(&mut &mut ctx.accounts.nft_marketplace_market_tokens.data.borrow_mut()[..])?;
    
        Ok(())
    }
    pub fn list_market_items(ctx: Context<ListMarketItems>, prices: Vec<u64>) -> ProgramResult {
        // Ensure the number of prices matches the number of NFTs.
        if prices.len() != ctx.accounts.nft_metadata_list.len() {
            return Err(ErrorCode::InvalidArgument.into());
        }
    
        // Iterate through a list of NFTs and create market tokens for each.
        for (nft_metadata, price) in ctx.accounts.nft_metadata_list.iter().zip(prices.into_iter()) {
            let token_id = nft_metadata.data.token_id;
            let market_token = MarketToken {
                item_id: token_id,
                nft_contract: *ctx.accounts.nft.to_account_info().key,
                token_id: token_id,
                seller: *ctx.accounts.creator.to_account_info().key,
                owner: *ctx.accounts.creator.to_account_info().key,
                price, // Set the price based on input.
                sold: false,
            };
    
            // Store the market token in the marketplace.
            let mut market_tokens = MarketTokens::from(&mut ctx.accounts.nft_marketplace_market_tokens)?;
            market_tokens.push(&market_token)?;
        }
    
        Ok(())
    }
    pub fn remove_market_item(ctx: Context<RemoveMarketItem>, item_id: u64) -> ProgramResult {
        // Find the market token by item_id.
        let index = next_market_token_id(ctx.accounts.nft_marketplace.to_account_info())?;
        let market_token_index = index.checked_sub(1).ok_or(ErrorCode::InvalidArgument)?;
        let mut market_tokens = MarketTokens::from(&mut ctx.accounts.nft_marketplace_market_tokens)?;
        let market_token = market_tokens
            .get(market_token_index as usize)
            .ok_or(ErrorCode::InvalidArgument)?;
    
        // Ensure the sender is the owner of the item.
        if ctx.accounts.seller.to_account_info().key != &market_token.owner {
            return Err(ErrorCode::Unauthorized.into());
        }
    
        // Remove the market item.
        market_tokens.remove(market_token_index as usize);
    
        Ok(())
    }
            
}

#[derive(Accounts)]
pub struct CreateMarketItem<'info> {
    #[account(init, payer = user, mint = mint, space = 165)]
    pub nft: Account<'info, Token>,
    #[account(signer)]
    pub mint_authority: AccountInfo<'info>,
    #[account(mut)]
    pub mint: AccountInfo<'info>,
    #[account(mut)]
    pub user: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
    pub token_metadata_program: AccountInfo<'info>,
    pub payer: AccountInfo<'info>,
    #[account(init, space = 8)]
    pub nft_metadata: Account<'info, TokenMetadata>,
    pub update_authority: AccountInfo<'info>,
    pub nft_authority: AccountInfo<'info>,
    pub nft_marketplace: AccountInfo<'info>,
    pub nft_marketplace_market_tokens: Account<'info, MarketTokens>,
    pub creator: AccountInfo<'info>,
}

#[account]
pub struct TokenMetadataV2 {
    pub data: Data,
    pub primary_sale_happened: bool,
    pub is_mutable: bool,
    pub mint: Pubkey,
    pub update_authority: Pubkey,
    pub token: Pubkey,
    pub seller_fee_collection_token: Pubkey,
    pub creators_configured: bool,
}

#[account(zero_copy)]
pub struct MarketTokens {
    pub data: Vec<u8>,
}

impl MarketTokens {
    pub fn from(data: &mut Account) -> Result<MarketTokens, ProgramError> {
        let mut market_tokens = MarketTokens { data: data.data.clone() };
        market_tokens.deserialize(data.data.borrow())?;
        Ok(market_tokens)
    }

    pub fn push(&mut self, market_token: &MarketToken) -> Result<(), ProgramError> {
        let mut market_tokens = self.deserialize(&self.serialize()?)?;
        market_tokens.push(market_token);
        self.data = market_tokens.serialize()?;
        Ok(())
    }
}
