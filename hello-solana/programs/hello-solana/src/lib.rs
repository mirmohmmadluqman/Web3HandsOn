use anchor_lang::prelude::*; // importing a toolkit for  macros, types, functions from the Anchor framework

declare_id!("EVgozyEhtWxfSLK5X5CrX5gGK7n8WjCLbncpH9XQDVE5");

#[program]
pub mod hello_solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);       
        Ok(())
    }

    pub fn initialize(ctx: Context<Update>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)] 
pub struct Initialize {}


#[derive(Accounts)]
pub struct Update {}