/*use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenInterface, TransferChecked, TokenAccount, CloseAccount, close_account},
};

use crate::state::Escrow;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Refund<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(
        mint::token_program = token_program,
    )]
    pub mint_a: InterfaceAccount<'info, Mint>,

    #[account(
        mut, 
        associated_token::mint = mint_a,
        associated_token::authority = maker, // maker is the authority of the ATA
        associated_token::token_program = token_program,
    )]
    pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        close = maker,
        has_one = mint_a, // making sure that the mint is the same as the one in the escrow using the has_one constraint
        has_one = maker,  // ensuring the maker in the escrow matches the signer
        seeds = [b"escrow", maker.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump = escrow.bump,
    )]
    pub escrow: Account<'info, Escrow>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow, // the vault is owned by the escrow account
        associated_token::token_program = token_program,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl <'info> Refund <'info> {
  pub fn refund_and_close_vault (&mut self) -> Result<()> {
    let cpi_program = self.token_program.to_account_info();

    let signer_seeds: [&[&[u8]]; 1] = [&[    // signer seeds for the PDA which is used to sign the transaction. 0 is the byte length of the PDA (known through the bump)
      b"escrow",
      self.maker.to_account_info().key.as_ref(),
      &self.escrow.seed.to_le_bytes()[..],
      &[self.escrow.bump],
    ]];
    // Transfer the tokens from the vault to the maker's ATA
    let transfer_accounts = TransferChecked {
      from: self.vault.to_account_info(),
      to: self.maker_ata_a.to_account_info(),
      mint: self.mint_a.to_account_info(),
      authority: self.escrow.to_account_info(),
    };

    let cpi_ctx = CpiContext::new_with_signer(cpi_program, transfer_accounts, &signer_seeds);

    transfer_checked(cpi_ctx, self.vault.amount, self.mint_a.decimals)?;

    // Close the vault account and send the remaining balance to the maker's ATA
    let close_cpi_program = self.token_program.to_account_info();
    let close_accounts = CloseAccount {
      account: self.vault.to_account_info(), // the account to close
      destination: self.maker_ata_a.to_account_info(), // the destination to send the remaining balance
      authority: self.escrow.to_account_info(), 
    };

    let close_cpi_ctx = CpiContext::new_with_signer(close_cpi_program, close_accounts, &signer_seeds);
    
    close_account(close_cpi_ctx)?;

    todo!()
  }
}*/
use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{close_account, transfer_checked, CloseAccount, TransferChecked},token_interface::{Mint, TokenAccount, TokenInterface}};

use crate::Escrow;


#[derive(Accounts)]
pub struct Refund<'info>{
    #[account(mut)]
    pub maker: Signer<'info>,
    pub mint_a: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = maker,
    )]
    pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        close = maker,
        seeds = [b"escrow", escrow.maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump,
        constraint = (maker.key() == escrow.maker.key()),
    )]
    pub escrow: Account<'info, Escrow>,
    
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl <'info>Refund<'info> {
    pub fn refund(&mut self)-> Result<()>{
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = TransferChecked{
            from: self.vault.to_account_info(),
            to: self.maker_ata_a.to_account_info(),
            mint: self.mint_a.to_account_info(),
            authority: self.escrow.to_account_info(),
        };

        let signer_seeds: [&[&[u8]]; 1] = [&[
            b"escrow",
            self.maker.to_account_info().key.as_ref(),
            &self.escrow.seed.to_le_bytes()[..],
            &[self.escrow.bump],
        ]];


        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, &signer_seeds);

        let amount = self.vault.amount;

        transfer_checked(cpi_ctx, amount, self.mint_a.decimals)?;

        Ok(())
    }

    pub fn close_refund(&mut self)-> Result<()>{
        let cpi_program =self.token_program.to_account_info();

        let cpi_accounts = CloseAccount{
            account: self.vault.to_account_info(),
            destination: self.maker.to_account_info(),
            authority: self.escrow.to_account_info(),
        };

        let signer_seeds: [&[&[u8]]; 1] = [&[
            b"escrow",
            self.maker.to_account_info().key.as_ref(),
            &self.escrow.seed.to_le_bytes()[..],
            &[self.escrow.bump],
        ]];


        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, &signer_seeds);

        close_account(cpi_ctx)?;

        Ok(())
    }
}
