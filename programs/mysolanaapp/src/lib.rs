use anchor_lang::prelude::*;
use solana_program::{
    entrypoint::ProgramResult
};


declare_id!("47hwQKg6fkRPXMkGSttDxhwtqzy7a6Epa9JGQFwBbDV8");

#[program]
pub mod mysolanaapp {
    use super::*;

    pub fn create(
        ctx: Context<Create>,
    ) ->  ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        base_account.count = 0;
        Ok(())
    }

    pub fn increment(
        ctx: Context<Increment>,
    ) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        base_account.count += 1;
        Ok(())
    }


}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer = user, space=16 + 16)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)] 
pub struct Increment<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
}

#[account] 
pub struct BaseAccount {
    pub count: u64,
}