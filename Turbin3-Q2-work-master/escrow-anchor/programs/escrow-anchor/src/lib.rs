use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

pub use instructions::*;
pub use state::*;

declare_id!("1JiPMFJx3kEjtFKB6KyAy3CkTo3SuGEwHSXhPRoG1QB");

#[program]
pub mod escrow_anchor {
    use super::*;

    pub fn make(ctx: Context<Make>, seed: u64, receive: u64) -> Result<()> {
        ctx.accounts.init_and_deposit_in_escrow(seed, receive, &ctx.bumps)?;
        Ok(())
    }

    pub fn take(ctx: Context<Take>) -> Result<()> {
        ctx.accounts.send_funds()?;
        ctx.accounts.withdraw()?;
        ctx.accounts.close()?;
        Ok(())
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        ctx.accounts.refund()?;
        ctx.accounts.close_refund()?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
