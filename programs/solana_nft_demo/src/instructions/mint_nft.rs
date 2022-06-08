use crate::{error::*, state::*};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::system_instruction;
use anchor_spl::token;
use anchor_spl::token::{MintTo, Token};
use mpl_token_metadata::instruction::{create_master_edition_v3, create_metadata_accounts_v2};

#[derive(Accounts)]
pub struct MintNFT<'info> {
    #[account(mut)]
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
    system_program: Program<'info, System>,
    rent: Sysvar<'info, Rent>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    master_edition: UncheckedAccount<'info>,
    #[account(mut, seeds = [ContractData::SEED], bump = contract_data.bump)]
    contract_data: Account<'info, ContractData>,
    #[account(init_if_needed,  seeds = [UserData::SEED, mint_authority.key().as_ref()], payer = mint_authority, bump, space = 8 + UserData::SPACE)]
    user_data: Account<'info, UserData>,
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
    // mint limit check
    require!(
        ctx.accounts.contract_data.total_minted < ContractData::LIMIT,
        self::CustomErrors::MintLimit
    );

    msg!("Transferring mint funds to treasury");
    transfer_lamports(
        ctx.accounts.mint_authority.to_account_info(),
        ctx.accounts.treasury.to_account_info(),
        ctx.accounts.contract_data.fee,
    )?;

    msg!("Initializing mint");
    // CPI program
    let cpi_program = ctx.accounts.token_program.to_account_info();
    // CPI accounts
    let token_mint = ctx.accounts.mint.to_account_info();
    let token_mint_id = token_mint.key;
    let cpi_accounts = MintTo {
        mint: token_mint,
        to: ctx.accounts.token_account.to_account_info(),
        authority: ctx.accounts.mint_authority.to_account_info(),
    };
    // CPI context assigned
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    let mint_amount = 1;
    token::mint_to(cpi_ctx, mint_amount)?;
    msg!("Minted token id: {}", token_mint_id);

    // token symbol
    let symbol = std::string::ToString::to_string("NUTS");
    // creator
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
    // account info
    let account_info = vec![
        ctx.accounts.metadata.to_account_info(),
        ctx.accounts.mint.to_account_info(),
        ctx.accounts.mint_authority.to_account_info(),
        ctx.accounts.mint_authority.to_account_info(),
        ctx.accounts.token_metadata_program.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.system_program.to_account_info(),
        ctx.accounts.rent.to_account_info(),
    ];
    invoke(
        &create_metadata_accounts_v2(
            ctx.accounts.token_metadata_program.key(),
            ctx.accounts.metadata.key(),
            ctx.accounts.mint.key(),
            ctx.accounts.mint_authority.key(),
            ctx.accounts.mint_authority.key(),
            ctx.accounts.mint_authority.key(),
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
    msg!("Metadata account created !!!");

    // master edition info
    let master_edition_infos = vec![
        ctx.accounts.master_edition.to_account_info(),
        ctx.accounts.mint.to_account_info(),
        ctx.accounts.mint_authority.to_account_info(),
        ctx.accounts.mint_authority.to_account_info(),
        ctx.accounts.metadata.to_account_info(),
        ctx.accounts.token_metadata_program.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.system_program.to_account_info(),
        ctx.accounts.rent.to_account_info(),
    ];
    invoke(
        &create_master_edition_v3(
            ctx.accounts.token_metadata_program.key(),
            ctx.accounts.master_edition.key(),
            ctx.accounts.mint.key(),
            ctx.accounts.mint_authority.key(),
            ctx.accounts.mint_authority.key(),
            ctx.accounts.metadata.key(),
            ctx.accounts.mint_authority.key(),
            Some(0),
        ),
        master_edition_infos.as_slice(),
    )?;
    msg!("Master edition nft minted !!!");

    // increase total minted amount
    ctx.accounts.contract_data.total_minted += 1;
    ctx.accounts.user_data.total_minted += 1;

    // save latest mint timestamp
    ctx.accounts.user_data.latest_mint_timestamp = Clock::get().unwrap().unix_timestamp as u32;
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
    let ix = system_instruction::transfer(&from.key(), &to.key(), amount);
    invoke(&ix, &[from, to])?;

    Ok(())
}
