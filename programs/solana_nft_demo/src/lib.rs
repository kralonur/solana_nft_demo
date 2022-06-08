use crate::instructions::*;
use anchor_lang::prelude::*;

pub mod error;
pub mod instructions;
pub mod state;

declare_id!("88pY5YqPPrQtBz8s154c5wENE6hRE6KVY9AJtobmeSJ8");

#[program]
pub mod solana_nft_demo {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, mint_fee: u64) -> Result<()> {
        instructions::initialize(ctx, mint_fee)
    }

    pub fn finalize(ctx: Context<Finalize>) -> Result<()> {
        instructions::finalize(ctx)
    }

    pub fn update_fee(ctx: Context<UpdateFee>, mint_fee: u64) -> Result<()> {
        instructions::update_fee(ctx, mint_fee)
    }

    pub fn mint_nft(
        ctx: Context<MintNFT>,
        creator_key: Pubkey,
        uri: String,
        title: String,
    ) -> Result<()> {
        instructions::mint_nft(ctx, creator_key, uri, title)
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        instructions::withdraw(ctx, amount)
    }
}
