use anchor_lang::prelude::*;

declare_id!("H4iPvKUYFtdis612fUaL8dxVD4HQF73YD2r4dsRYZL6P");

pub mod state;
pub mod context;

pub use state::*;
pub use context::*;

#[program]
pub mod escrow_donate {
    use super::*;

    pub fn make(ctx: Context<Make>, amount: u64) -> Result<()> {
        ctx.accounts.make(amount, &ctx.bumps)?;

        Ok(())
    }

    pub fn donate(ctx: Context<Donate>, amount: u64) -> Result<()> {
        ctx.accounts.donate(amount)?;

        Ok(())
    }

    pub fn check_donations(ctx: Context<Donate>) -> Result<()> {
        ctx.accounts.check_donations()?;

        Ok(())
    }
}