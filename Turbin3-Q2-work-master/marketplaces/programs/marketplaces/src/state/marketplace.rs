use anchor_lang::prelude::*;

// This is the core state of core accounts for configuring the marketplace
#[account]
pub struct Marketplace {
    pub admin: Pubkey, // the person who created the marketplace
    pub fee: u16, // the fee for the marketplace, in basis points
    pub bump: u8, // the bump for the marketplace account
    pub treasury_bump: u8, // the treasury is an account that holds the fees for the marketplace
    pub rewards_bump: u8,
    pub name: String,
}

impl Space for Marketplace {
    const INIT_SPACE: usize = 8  // anchor discriminator
                            + 32 // admin
                            + 2 // fee
                             + 3 * 1  // bump
                             + (4 + 32); // 4 is the basic allocation for a string (and vector), 32 is the max length of the string to be stored onchain
}