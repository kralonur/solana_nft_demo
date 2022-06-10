use crate::{error::*, state::*};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut, seeds = [ContractData::SEED], bump = contract_data.bump)]
    contract_data: Account<'info, ContractData>,
    /// CHECK:
    #[account(mut, seeds = [TREASURY_SEED], bump = contract_data.treasury_bump)]
    treasury: UncheckedAccount<'info>,
    #[account(mut, address = contract_data.authority)]
    authority: Signer<'info>,
    system_program: Program<'info, System>,
}

pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    // Warning: code is not safe !!!
    let from = &ctx.accounts.treasury.to_account_info();
    let to = &ctx.accounts.authority.to_account_info();

    let rent = Rent::get()?.minimum_balance(0);
    let available_amount_to_withdraw =
        **ctx.accounts.treasury.to_account_info().lamports.borrow() - rent;

    require!(
        available_amount_to_withdraw > 0,
        self::CustomErrors::RequestRentSol
    );
    require!(
        amount <= available_amount_to_withdraw,
        self::CustomErrors::InsufficientFunds
    );

    // Debit from_account and credit to_account
    transfer_lamports(from, to, amount)?;

    emit!(Withdrawn {
        amount,
        authority: ctx.accounts.authority.key()
    });

    Ok(())
}

fn transfer_lamports(from: &AccountInfo, to: &AccountInfo, amount: u64) -> Result<()> {
    **from.try_borrow_mut_lamports()? -= amount;
    **to.try_borrow_mut_lamports()? += amount;

    Ok(())
}

#[event]
struct Withdrawn {
    amount: u64,
    authority: Pubkey,
}
