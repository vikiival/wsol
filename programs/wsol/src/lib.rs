use anchor_lang::{prelude::*, system_program::Transfer};
use anchor_spl::token::{Mint, SyncNative, Token, TokenAccount};

pub const WSOL: Pubkey = pubkey!("So11111111111111111111111111111111111111112");

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("Auqt1ZFJEy2WPHXRJ94vanAqg7EYFui4hfcHJzhKqR7P");

#[program]
mod wsol_wrap_master {
    use super::*;
    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn wrap_sol(ctx: Context<WrapSol>, amount: u64) -> Result<()> {
        // Transfer SOL to WSOL account
        anchor_lang::system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.payer.to_account_info(),
                    to: ctx.accounts.wrap_sol_account.to_account_info(),
                },
            ),
            amount,
        )?;

        // 2. sync native
        let ix = anchor_spl::token::sync_native(CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            SyncNative {
                account: ctx.accounts.wrap_sol_account.to_account_info(),
            },
        ));

        ix?;

        Ok(())
    }

}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct WrapSol<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = wsol_mint,
        associated_token::authority = payer
    )]
    pub wrap_sol_account: Account<'info, TokenAccount>,

    #[account(address = WSOL)]
    pub wsol_mint: Account<'info, Mint>,

    /// Token program.
    pub token_program: Program<'info, Token>,

    /// System program.
    pub system_program: Program<'info, System>,

    pub associated_token_program: Program<'info, anchor_spl::associated_token::AssociatedToken>,
}