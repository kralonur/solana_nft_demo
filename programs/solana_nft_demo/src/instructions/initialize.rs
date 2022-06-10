use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, seeds = [ContractData::SEED], bump, payer = authority, space = 8 + ContractData::SPACE)]
    contract_data: Account<'info, ContractData>,
    /// CHECK:
    #[account(init, seeds = [TREASURY_SEED], bump, payer = authority, space = 0)]
    treasury: UncheckedAccount<'info>,
    #[account(mut)]
    authority: Signer<'info>,
    system_program: Program<'info, System>,
}

pub fn initialize(ctx: Context<Initialize>, mint_fee: u64) -> Result<()> {
    let contract_data = &mut ctx.accounts.contract_data;

    contract_data.bump = *ctx.bumps.get("contract_data").unwrap();
    contract_data.treasury_bump = *ctx.bumps.get("treasury").unwrap();
    contract_data.authority = ctx.accounts.authority.key();
    contract_data.fee = mint_fee;

    emit!(Initialized { fee: mint_fee });
    
    Ok(())
}

#[event]
struct Initialized {
    fee: u64,
}
