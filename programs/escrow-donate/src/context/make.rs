use anchor_spl::{associated_token::AssociatedToken, token::{Mint, TokenAccount, Token}};

use crate::*;

#[derive(Accounts)]
pub struct Make<'info> {
    #[account(mut)]
    signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        seeds = [b"escrow-donate", signer.key().as_ref()],
        bump,
        space = EscrowDonate::INIT_SPACE,
    )]
    pub escrow: Account<'info, EscrowDonate>,
    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = escrow,
    )]
    pub vault_ata: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = signer,
    )]
    maker_ata: Account<'info, TokenAccount>,
    #[account(mut)]
    mint: Account<'info, Mint>,
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
    associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> Make<'info> {
    pub fn make(&mut self, amount: u64, bumps: &MakeBumps) -> Result<()> {
        self.escrow.mint = self.mint.key();
        self.escrow.target = amount;
        self.escrow.bump = bumps.escrow;

        msg!("Creating escrow account");
        msg!("Escrow mint: {}", self.escrow.mint);
        msg!("Escrow maker ATA: {}", self.maker_ata.key());
        msg!("Escrow target amount: {}", self.escrow.target);

        Ok(())
    }
}