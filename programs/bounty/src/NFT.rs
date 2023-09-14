use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, Transfer};
use anchor_spl::token_metadata::{self, TokenMetadata, UpdateAuthority};

#[program]
mod nft_marketplace {
    use super::*;

    pub fn create_nft(ctx: Context<CreateNFT>, token_uri: String) -> ProgramResult {
        let mint_authority = &ctx.accounts.mint_authority;

        // Create a new NFT.
        let token_metadata = TokenMetadata {
            data: token_metadata::Data {
                name: Some("MoonBirdz".to_string()),
                symbol: Some("MBIRDZ".to_string()),
                uri: token_uri,
                seller_fee_basis_points: 500, // Example seller fee.
                creators: Some(vec![token_metadata::Creator {
                    address: ctx.accounts.creator.key().clone(),
                    verified: true,
                    share: 10000, // 100% share to the creator.
                }]),
            },
            update_authority: ctx.accounts.update_authority.key(),
            primary_sale_happened: false,
            is_mutable: true,
            mint: ctx.accounts.mint.key(),
            token: ctx.accounts.nft.to_account_info().key(),
            seller_fee_collection_token: ctx.accounts.seller_fee.to_account_info().key(),
            creators_configured: true,
        };

        let cpi_accounts = Transfer {
            from: ctx.accounts.mint_authority.to_account_info(),
            to: ctx.accounts.nft.to_account_info(),
            authority: ctx.accounts.nft_authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.clone();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, 1)?;

        let cpi_accounts = token_metadata::CreateMetadata {
            metadata: ctx.accounts.nft_metadata.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            mint_authority: ctx.accounts.mint_authority.to_account_info(),
            payer: ctx.accounts.payer.to_account_info(),
            update_authority: ctx.accounts.update_authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_metadata_program.clone();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token_metadata::create_metadata(cpi_ctx, token_metadata)?;

        Ok(())
    }

    // Other marketplace operations can be added here.
}

#[derive(Accounts)]
pub struct CreateNFT<'info> {
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
    #[account(init, mint = mint, owner = payer, space = 8)]
    pub seller_fee: Account<'info, Token>,
    pub creator: AccountInfo<'info>,
}

#[account]
pub struct TokenMetadataV2 {
    pub data: token_metadata::Data,
    pub primary_sale_happened: bool,
    pub is_mutable: bool,
    pub mint: Pubkey,
    pub update_authority: Pubkey,
    pub token: Pubkey,
    pub seller_fee_collection_token: Pubkey,
    pub creators_configured: bool,
}
