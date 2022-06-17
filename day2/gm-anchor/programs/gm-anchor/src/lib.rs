use anchor_lang::prelude::*;

declare_id!("GTbyVP5vN4p7JekqFRGJAoPbTUGzUMvLTxfNiSREafZo");

#[program]
pub mod gm_anchor {
    use super::*;
    // defines an execute function which takes an account from the context and a 'name' parameter
    pub fn execute(ctx: Context<Execute>, name: String) -> Result<()> {
        // stores the name string into the specified account
        let gm_account = &mut ctx.accounts.gm_account;
        gm_account.name = name;
        // prints the name out the program output
        msg!("GM {}", gm_account.name);
        Ok(())
    }
}

// struct that defines the accounts passed into the execute instruction, and the deserialization of the gm_account account into a 'GreetingAccount' struct
#[derive(Accounts)]
pub struct Execute<'info> {
    #[account(init, payer = user, space = 8+32)]
    pub gm_account: Account<'info, GreetingAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// defines the 'GreetingAccount' struct that stores the name string
#[account]
pub struct GreetingAccount {
    pub name: String,
}