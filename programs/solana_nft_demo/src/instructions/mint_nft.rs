use crate::{error::*, state::*};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_spl::token;
use anchor_spl::token::{MintTo, Token};
use mpl_token_metadata::instruction::{create_master_edition_v3, create_metadata_accounts_v2};

#[derive(Accounts)]
pub struct MintNFT<'info> {
    #[account(mut, address = contract_data.authority)]
    mint_authority: Signer<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    mint: UncheckedAccount<'info>,
    token_program: Program<'info, Token>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    metadata: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    token_account: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    token_metadata_program: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut, address = contract_data.authority)]
    payer: UncheckedAccount<'info>,
    system_program: Program<'info, System>,
    rent: Sysvar<'info, Rent>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    master_edition: UncheckedAccount<'info>,
    #[account(mut, seeds = [ContractData::SEED], bump = contract_data.bump)]
    contract_data: Account<'info, ContractData>,
    /// CHECK:
    #[account(mut, seeds = [TREASURY_SEED], bump = contract_data.treasury_bump)]
    treasury: UncheckedAccount<'info>,
}

pub fn mint_nft(
    ctx: Context<MintNFT>,
    creator_key: Pubkey,
    uri: String,
    title: String,
) -> Result<()> {
    require!(
        ctx.accounts.contract_data.total_minted < ContractData::LIMIT,
        self::CustomErrors::MintLimit
    );

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
        ctx.accounts.payer.to_account_info(),
        ctx.accounts.treasury.to_account_info(),
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

    // increase total minted amount
    ctx.accounts.contract_data.total_minted = ctx.accounts.contract_data.total_minted + 1;
    msg!("Master Edition Nft Minted !!!");
    emit!(NFTMinted {
        nft_num: ctx.accounts.contract_data.total_minted
    });
    Ok(())
}

#[event]
struct NFTMinted {
    nft_num: u16,
}

fn transfer_lamports<'a>(from: AccountInfo<'a>, to: AccountInfo<'a>, amount: u64) -> Result<()> {
    let ix =
        anchor_lang::solana_program::system_instruction::transfer(&from.key(), &to.key(), amount);
    anchor_lang::solana_program::program::invoke(
        &ix,
        &[from.to_account_info(), to.to_account_info()],
    )?;

    Ok(())
}
