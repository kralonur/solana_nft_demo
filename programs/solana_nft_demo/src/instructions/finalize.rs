use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Finalize<'info> {
    #[account(mut, seeds = [ContractData::SEED], bump = contract_data.bump, close = authority)]
    contract_data: Account<'info, ContractData>,
    #[account(mut, seeds = [UserData::SEED, authority.key().as_ref()], bump, close = authority)]
    user_data: Account<'info, UserData>,
    /// CHECK:
    #[account(mut, seeds = [TREASURY_SEED], bump = contract_data.treasury_bump)]
    treasury: UncheckedAccount<'info>,
    #[account(mut, address = contract_data.authority)]
    authority: Signer<'info>,
    system_program: Program<'info, System>,
}

pub fn finalize(ctx: Context<Finalize>) -> Result<()> {
    let from = &ctx.accounts.treasury.to_account_info();
    let to = &ctx.accounts.authority.to_account_info();

    **to.try_borrow_mut_lamports()? += from.lamports();
    **from.try_borrow_mut_lamports()? = 0;

    emit!(Finalized {
        authority: ctx.accounts.authority.key()
    });
    Ok(())
}

#[event]
struct Finalized {
    authority: Pubkey,
}
