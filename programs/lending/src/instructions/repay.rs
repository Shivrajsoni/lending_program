use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{ self, Mint, TokenAccount, TokenInterface, TransferChecked };
use crate::state::*;
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct Repay <'info> {

    #[account(mut)]
    pub signer:Signer<'info>,
    pub mint:InterfaceAccount<'info,Mint>,

    #[account(
        mut,
        seeds = [mint.key().as_ref()],
        bump
    )]
    pub bank:Account<'info,Bank>,

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

    pub user : Account<'info,User>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = signer,
        associated_token::token_program = token_program,
    )]
    
    pub user_token_account:InterfaceAccount<'info,TokenAccount>,
    pub system_program:Program<'info,System>,
    pub token_program:Interface<'info,TokenInterface>,
    pub associated_token_program : Program<'info,AssociatedToken>
}


pub fn process_repay(ctx:Context<Repay>,amount:u64) -> Result<()> {
    let user = &mut ctx.accounts.user;
    let borrowed_asset ;

    // Note: For simplicity, interest fees are not included in this calculation
    match ctx.accounts.mint.to_account_info().key() {
        key if key == user.usdc_address => {
            borrowed_asset = user.borrowed_usdc;
        },
        _=> {
            borrowed_asset = user.borrowed_sol;
        }
    }

    if amount >borrowed_asset {
        return Err(ErrorCode::OverRepay.into());
    }

    let transfer_cpi_program = TransferChecked{
        from:ctx.accounts.user_token_account.to_account_info(),
        to:ctx.accounts.bank_token_account.to_account_info(),
        mint:ctx.accounts.mint.to_account_info(),
        authority:ctx.accounts.user_token_account.to_account_info(),
    }; 

    let cpi_program = ctx.accounts.token_program.to_account_info();

    let cpi_context = CpiContext::new(cpi_program, transfer_cpi_program);

    let decimals = ctx.accounts.mint.decimals;

    token_interface::transfer_checked(cpi_context, amount, decimals)?;

    let bank = &mut ctx.accounts.bank;


    let borrowed_ratio = amount.checked_div(bank.total_borrowed).unwrap();
    let users_shares = bank.total_borrowed_shares.checked_mul(borrowed_ratio).unwrap();

    
    match ctx.accounts.mint.to_account_info().key() {
        key if key == user.usdc_address => {
            user.borrowed_usdc -= amount;
            user.borrowed_usdc_shares -= users_shares;
        },
        _ => {
            user.borrowed_sol -= amount;
            user.borrowed_sol_shares -= users_shares; 
        }
    }

    // Add in "update health factor" function here

    bank.total_borrowed -= amount;
    bank.total_borrowed_shares -= users_shares;
    
    
    Ok(())
}