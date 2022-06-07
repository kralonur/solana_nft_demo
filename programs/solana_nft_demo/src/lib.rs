use crate::instructions::*;
use anchor_lang::prelude::*;

pub mod error;
pub mod instructions;
pub mod state;

declare_id!("8WK7ZXXsqbx5j1We5pE6DYczTfE2vzu5qoQd2rb1sfUK");

#[program]
pub mod solana_nft_demo {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, mint_fee: u64) -> Result<()> {
        instructions::initialize(ctx, mint_fee)
    }

    pub fn finalize(_ctx: Context<Finalize>) -> Result<()> {
        instructions::finalize(_ctx)
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
