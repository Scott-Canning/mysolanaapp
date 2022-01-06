use anchor_lang::prelude::*;

declare_id!("yuVKvPgp5CjrdM5xbjo5U1DePwN9Qwoc8gBLFc7cf9Z");

#[program]
pub mod mysolanaapp {
    use super::*;

    // create RPC handler, passing Create ctx (base_account, user, system_program)
    pub fn create(ctx: Context<Create>) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        base_account.count = 0;
        Ok(())
    }

    // increment RPC handler, passing Increment ctx (base_account)
    pub fn increment(ctx: Context<Increment>) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        base_account.count += 1;
        Ok(())
    }

}

// transaction instructions
#[derive(Accounts)]
pub struct Create<'info> {
    // [account(...)] defines constraints; if not followed, instructions won't execute
    #[account(init, payer = user, space = 16 + 16)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// transaction instructions
#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
}

// an account that goes inside a transaction instruction
#[account]
pub struct BaseAccount {
    pub count: u64,
}
