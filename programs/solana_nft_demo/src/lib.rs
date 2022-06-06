use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_spl::token;
use anchor_spl::token::{MintTo, Token};
use mpl_token_metadata::instruction::{create_master_edition_v3, create_metadata_accounts_v2};

declare_id!("8WK7ZXXsqbx5j1We5pE6DYczTfE2vzu5qoQd2rb1sfUK");

pub const CONTRACT_DATA_SEED: &[u8] = b"contractdata";
pub const TREASURY_SEED: &[u8] = b"treasury";

#[program]
pub mod solana_nft_demo {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, mint_fee: u64) -> Result<()> {
        let contract_data = &mut ctx.accounts.contract_data;

        contract_data.bump = *ctx.bumps.get("contract_data").unwrap();
        contract_data.treasury_bump = *ctx.bumps.get("treasury").unwrap();
        contract_data.authority = *ctx.accounts.authority.key;
        contract_data.fee = mint_fee;

        Ok(())
    }

    pub fn finalize(_ctx: Context<Finalize>) -> Result<()> {
        Ok(())
    }

    pub fn update_fee(ctx: Context<UpdateFee>, mint_fee: u64) -> Result<()> {
        let contract_data = &mut ctx.accounts.contract_data;
        contract_data.fee = mint_fee;
        Ok(())
    }

    pub fn mint_nft(
        ctx: Context<MintNFT>,
        creator_key: Pubkey,
        uri: String,
        title: String,
    ) -> Result<()> {
        msg!("Initializing Mint Ticket");
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
        };
        // TEST PART !!!!!
        // WORKING!!!
        // let from = ctx.accounts.payer.key();
        // let to = ctx.accounts.treasury.key();
        // let amount_of_lamports: u64 = ctx.accounts.contract_data.fee;
        // let ix = anchor_lang::solana_program::system_instruction::transfer(
        //     &from,
        //     &to,
        //     amount_of_lamports,
        // );
        // anchor_lang::solana_program::program::invoke(
        //     &ix,
        //     &[
        //         ctx.accounts.payer.to_account_info(),
        //         ctx.accounts.treasury.to_account_info(),
        //     ],
        // )?;
        transfer_lamports(
            &mut ctx.accounts.payer,
            &mut ctx.accounts.treasury,
            ctx.accounts.contract_data.fee,
        )?;
        // WORKING!!!
        // // NOT_WORKING!!!
        // let from = &ctx.accounts.mint_authority.to_account_info();
        // let to = &ctx.accounts.treasury.to_account_info();
        // let amount: u64 = ctx.accounts.contract_data.fee;
        // if **from.try_borrow_lamports()? < amount {
        //     panic!("no sol");
        // }
        // // Debit from_account and credit to_account
        // **from.try_borrow_mut_lamports()? -= amount;
        // **to.try_borrow_mut_lamports()? += amount;
        // // NOT_WORKING!!!
        // TEST PART !!!!!
        msg!("CPI Accounts Assigned");
        let cpi_program = ctx.accounts.token_program.to_account_info();
        msg!("CPI Program Assigned");
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        msg!("CPI Context Assigned");
        token::mint_to(cpi_ctx, 1)?;
        msg!("Token Minted !!!");
        let account_info = vec![
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.mint_authority.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.token_metadata_program.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ];
        msg!("Account Info Assigned");
        let creator = vec![
            mpl_token_metadata::state::Creator {
                address: creator_key,
                verified: false,
                share: 100,
            },
            mpl_token_metadata::state::Creator {
                address: ctx.accounts.mint_authority.key(),
                verified: false,
                share: 0,
            },
        ];
        msg!("Creator Assigned");
        let symbol = std::string::ToString::to_string("symb");
        invoke(
            &create_metadata_accounts_v2(
                ctx.accounts.token_metadata_program.key(),
                ctx.accounts.metadata.key(),
                ctx.accounts.mint.key(),
                ctx.accounts.mint_authority.key(),
                ctx.accounts.payer.key(),
                ctx.accounts.payer.key(),
                title,
                symbol,
                uri,
                Some(creator),
                1,
                true,
                false,
                None,
                None,
            ),
            account_info.as_slice(),
        )?;
        msg!("Metadata Account Created !!!");
        let master_edition_infos = vec![
            ctx.accounts.master_edition.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.mint_authority.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.token_metadata_program.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ];
        msg!("Master Edition Account Infos Assigned");
        invoke(
            &create_master_edition_v3(
                ctx.accounts.token_metadata_program.key(),
                ctx.accounts.master_edition.key(),
                ctx.accounts.mint.key(),
                ctx.accounts.payer.key(),
                ctx.accounts.mint_authority.key(),
                ctx.accounts.metadata.key(),
                ctx.accounts.payer.key(),
                Some(0),
            ),
            master_edition_infos.as_slice(),
        )?;
        msg!("Master Edition Nft Minted !!!");
        Ok(())
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

        let rent = Rent::get()?.minimum_balance(ctx.accounts.treasury.to_account_info().data_len());
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
}

pub fn transfer_lamports<'a>(
    from: &mut AccountInfo<'a>,
    to: &mut Account<'a, Treasury>,
    amount: u64,
) -> Result<()> {
    let ix =
        anchor_lang::solana_program::system_instruction::transfer(&from.key(), &to.key(), amount);
    anchor_lang::solana_program::program::invoke(
        &ix,
        &[from.to_account_info(), to.to_account_info()],
    )?;

    Ok(())
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, seeds = [CONTRACT_DATA_SEED], bump, payer = authority, space = 50)]
    pub contract_data: Account<'info, ContractData>,
    #[account(init, seeds = [TREASURY_SEED], bump, payer = authority, space = 8)]
    pub treasury: Account<'info, Treasury>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Finalize<'info> {
    #[account(mut, seeds = [CONTRACT_DATA_SEED], bump = contract_data.bump, close = authority)]
    pub contract_data: Account<'info, ContractData>,
    #[account(mut, seeds = [TREASURY_SEED], bump = contract_data.treasury_bump, close = authority)]
    pub treasury: Account<'info, Treasury>,
    #[account(mut, address = contract_data.authority)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateFee<'info> {
    #[account(mut, seeds = [CONTRACT_DATA_SEED], bump = contract_data.bump)]
    pub contract_data: Account<'info, ContractData>,
    #[account(mut, address = contract_data.authority)]
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct MintNFT<'info> {
    #[account(mut, address = contract_data.authority)]
    pub mint_authority: Signer<'info>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub mint: UncheckedAccount<'info>,
    // #[account(mut)]
    pub token_program: Program<'info, Token>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_metadata_program: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut, address = contract_data.authority)]
    pub payer: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub rent: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub master_edition: UncheckedAccount<'info>,
    #[account(seeds = [CONTRACT_DATA_SEED], bump = contract_data.bump)]
    pub contract_data: Account<'info, ContractData>,
    #[account(mut, seeds = [TREASURY_SEED], bump = contract_data.treasury_bump)]
    pub treasury: Account<'info, Treasury>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut, seeds = [CONTRACT_DATA_SEED], bump = contract_data.bump)]
    pub contract_data: Account<'info, ContractData>,
    #[account(mut, seeds = [TREASURY_SEED], bump = contract_data.treasury_bump)]
    pub treasury: Account<'info, Treasury>,
    #[account(mut, address = contract_data.authority)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ContractData {
    pub bump: u8,
    pub treasury_bump: u8,
    pub authority: Pubkey,
    pub fee: u64,
}

#[account]
pub struct Treasury {}

#[error_code]
pub enum CustomErrors {
    // 6000 0x1770
    #[msg("SOLs is not enough")]
    RequestRentSol,
    // 6001 0x1771
    #[msg("SOLs is not enough")]
    InsufficientFunds,
}
