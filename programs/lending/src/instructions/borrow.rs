use std::f32::consts::E;

use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{ self, Mint, TokenAccount, TokenInterface, TransferChecked };
use pyth_solana_receiver_sdk::price_update::{get_feed_id_from_hex, PriceUpdateV2};
use crate::constant::{MAXIMUM_AGE, SOL_USD_FEED_ID, USDC_USD_FEED_ID};
use crate::state::*;
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct Borrow <'info>{
    #[account(mut)]
    pub signer:Signer<'info>,
    pub mint:InterfaceAccount<'info,Mint>,

    #[account(
        mut,
        seeds = [mint.key().as_ref()],
        bump
    )]
    pub bank_account :Account<'info,Bank>,

    #[account(
        mut,
        seeds = [b"treasury",mint.key().as_ref()],
        bump
    )]
    pub bank_token_account:InterfaceAccount<'info,TokenAccount>,

    #[account(
        mut,
        seeds = [signer.key().as_ref()],
        bump
    )]
    pub user_account:Account<'info,User>,

    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = signer,
        associated_token::token_program = token_program
    )]
    

    pub user_token_account:InterfaceAccount<'info,TokenAccount>,
    pub price_update:Account<'info,PriceUpdateV2>,
    pub system_program:Program<'info,System>,
    pub token_program:Interface<'info,TokenInterface>,
    pub associated_token_program:Program<'info,AssociatedToken>
}

// 1. Check if user has enough collateral to borrow
// 2. Warn if borrowing beyond the safe amount but still allow if within the max borrowable amount
// 3. Make a CPI transfer from the bank's token account to the user's token account
// 4. Update the user's borrowed amount and total borrowed value
// 5. Update the bank's total borrows and total borrow shares

pub fn process_borrow(ctx:Context<Borrow>,amount:u64)->Result<()> {
    Ok(())
}