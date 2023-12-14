use crate::*;

#[account]
pub struct EscrowDonate {
    pub mint: Pubkey,
    //pub maker_ata: Pubkey,
    pub target: u64,
    pub bump: u8,
}

impl Space for EscrowDonate {
    const INIT_SPACE: usize = 8 + 32 + 8 + 1 ;
}