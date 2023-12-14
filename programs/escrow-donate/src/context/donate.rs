use anchor_spl::{token::{Token, TokenAccount, Mint, Transfer, transfer}, associated_token::AssociatedToken};
use anchor_lang::solana_program::sysvar::instructions as tx_instructions;

use crate::*;

#[derive(Accounts)]
pub struct Donate<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub owner: SystemAccount<'info>,
    #[account(
        mut,
        has_one = mint,
        seeds = [b"escrow-donate", owner.key().as_ref()],
        bump = escrow.bump,
    )]
    pub escrow: Account<'info, EscrowDonate>,
    #[account(
        mut,
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
    pub donator_ata: Account<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = owner,
    )]
    pub maker_ata: Account<'info, TokenAccount>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    /* Challenge */
    #[account(address = tx_instructions::ID)]
    /// CHECK: This is not dangerous as it is only the Sysvar account
    pub instructions: UncheckedAccount<'info>,
    #[account(mut)]
    pub rewards_mint: Account<'info, Mint>,
    /* Challenge */
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> Donate<'info> {
    pub fn donate(&mut self, amount: u64) -> Result<()> {
        let total_donated = self.maker_ata.amount;
        let remaining = self.escrow.target - total_donated;

        let amount_to_transfer = match amount > remaining {
            true => remaining,
            false => amount,
        };

        let cpi_program = self.token_program.to_account_info().clone();
        let cpi_accounts = Transfer {
            from: self.donator_ata.to_account_info(),
            to: self.vault_ata.to_account_info(),
            authority: self.signer.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_ctx, amount_to_transfer)?;

        msg!("Transferred {:?} tokens to Vault", amount_to_transfer);
        msg!("Vault ATA amount: {}", self.vault_ata.amount);

        //self.introspection()?; //Chalenge

        Ok(())
    }

    pub fn check_donations(&mut self) -> Result<()> {

        msg!("Checking donations");
        msg!("Vault ATA amount: {}", self.vault_ata.amount);
        msg!("Escrow target: {}", self.escrow.target);
        
        match self.vault_ata.amount >= self.escrow.target {
            true => {
                let seeds = &[
                    b"escrow-donate",
                    self.owner.key.as_ref(),
                    &[self.escrow.bump],
                ];
                let signer_seeds = &[&seeds[..]];

                let cpi_program = self.token_program.to_account_info().clone();
                let cpi_accounts = Transfer {
                    from: self.vault_ata.to_account_info(),
                    to: self.maker_ata.to_account_info(),
                    authority: self.escrow.to_account_info(),
                };
                let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

                transfer(cpi_ctx, self.escrow.target)?;

                msg!("Donations sent to maker!");

                self.close_accounts()?; // Challenge
            },
            false => msg!("Not enough donations!"),
        }
        Ok(())
    }

    pub fn close_accounts(&mut self) -> Result<()> {
        
        // Challenge

        Ok(())
    }

    pub fn introspection(&mut self) -> Result<()> {
        let ixns = self.instructions.to_account_info();
        let current_index = tx_instructions::load_current_index_checked(&ixns)? as usize;
        let current_ixn = tx_instructions::load_instruction_at_checked(current_index, &ixns)?;

        let program_id = current_ixn.program_id;
        let discriminator = &current_ixn.data[0..8];

        let next_ixn = tx_instructions::load_instruction_at_checked(current_index + 1, &ixns);
        match next_ixn {
            Ok(ixn) => {
                let next_program_id = ixn.program_id;
                let next_discriminator = &ixn.data[0..8];

                if program_id == next_program_id && discriminator == next_discriminator {
                    //Mint reward tokens
                }
            },
            Err(_) => msg!("No next instruction! No rewards minted!"),
        }

        Ok(())
    }
}