use anchor_lang::prelude::*;

declare_id!("9px1erHuqV9jHAAncku7omwMHNR7YkvugHET2tJa6MF8");

#[program]
pub mod solana_nft_demo {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
