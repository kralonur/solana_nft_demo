use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Finalize<'info> {
    #[account(mut, seeds = [ContractData::SEED], bump = contract_data.bump, close = authority)]
    contract_data: Account<'info, ContractData>,
    #[account(mut, seeds = [Treasury::SEED], bump = contract_data.treasury_bump, close = authority)]
    treasury: Account<'info, Treasury>,
    #[account(mut, address = contract_data.authority)]
    authority: Signer<'info>,
    system_program: Program<'info, System>,
}

pub fn finalize(_ctx: Context<Finalize>) -> Result<()> {
    Ok(())
}
