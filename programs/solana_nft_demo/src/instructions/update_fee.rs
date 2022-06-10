use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct UpdateFee<'info> {
    #[account(mut, seeds = [ContractData::SEED], bump = contract_data.bump)]
    contract_data: Account<'info, ContractData>,
    #[account(mut, address = contract_data.authority)]
    authority: Signer<'info>,
}

pub fn update_fee(ctx: Context<UpdateFee>, mint_fee: u64) -> Result<()> {
    let contract_data = &mut ctx.accounts.contract_data;
    contract_data.fee = mint_fee;

    emit!(FeeUpdated { fee: mint_fee });
    
    Ok(())
}

#[event]
struct FeeUpdated {
    fee: u64,
}
