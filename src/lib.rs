use anchor_lang::prelude::*;
use solana_program::entrypoint::ProgramResult;

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("7oPuKUVSL8A89Kurw1GD56ECaS1iQrkeEFyJnhqg59ij");

#[program]
mod counter_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        let counter_account = &mut ctx.accounts.counter_account;
        counter_account.count = 0;
        msg!("Counter created with an initial count of {}",counter_account.count);
        Ok(())
    }
    pub fn increase(ctx: Context<Increase>, increment: u64) -> ProgramResult {
        let counter_account = &mut ctx.accounts.counter_account;
        let current_count = &counter_account.count;
        counter_account.count = current_count+increment;
        msg!("Counter increased to: {}", counter_account.count);
        Ok(())
    }
    pub fn decrease(ctx: Context<Decrease>, decrement: u64) -> ProgramResult {
        let counter_account = &mut ctx.accounts.counter_account;
        let current_count = &counter_account.count;
        counter_account.count = current_count - decrement;
        msg!("Counter decreased to: {}", counter_account.count);
        Ok(())
    }
}
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub counter_account: Account<'info, Counter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increase<'info> {
    #[account(mut)]
    pub counter_account: Account<'info, Counter>,
}

#[derive(Accounts)]
pub struct Decrease<'info> {
    #[account(mut)]
    pub counter_account: Account<'info, Counter>,
}

#[account]
pub struct Counter {
    pub count: u64,
}