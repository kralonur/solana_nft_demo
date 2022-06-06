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
    // NOT_WORKING!!!
    // let from = ctx.accounts.treasury.key();
    // let to = ctx.accounts.authority.key();

    // let ix = anchor_lang::solana_program::system_instruction::transfer(&from, &to, amount);
    // anchor_lang::solana_program::program::invoke_signed(
    //     &ix,
    //     &[
    //         ctx.accounts.treasury.to_account_info(),
    //         ctx.accounts.authority.to_account_info(),
    //     ],
    //     &[&[
    //         &TREASURY_SEED[..],
    //         &[ctx.accounts.contract_data.treasury_bump],
    //     ]],
    // )?;
    // NOT_WORKING!!!

    // Warning: code is not safe !!!

    // WORKING!!!
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
    **from.try_borrow_mut_lamports()? -= amount;
    **to.try_borrow_mut_lamports()? += amount;
    // WORKING!!!

    Ok(())
}
