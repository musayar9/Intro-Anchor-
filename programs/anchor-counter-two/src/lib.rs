use anchor_lang::prelude::*;

declare_id!("2pHHxrFzCqXdiNVqL4Noi25Q7tpnPnK9nMnkAxmYrF5U");

#[program]
pub mod anchor_counter_two {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()>{
        let counter = &mut ctx.accounts.counter;
        counter.count = 0;
        msg!("Counter Account Created");
        msg!("Counter Count { }",  counter.count);
         Ok(())

    }

    pub fn increment(ctx: Context<Update>) -> Result<()>{
        let counter = &mut ctx.accounts.counter;
        msg!("Previos Counter: {}", counter.count);
        counter.count = counter.count.checked_add(1).unwrap();
        msg!("Counter incremented. Current Count: {}", counter.count);
        Ok(())

    }


    pub fn decrement(ctx:Context<Update>) -> Result<()>{
        let counter = &mut ctx.accounts.counter;
        msg!("previos counter: {}", counter.count);
        counter.count = counter.count.checked_sub(1).unwrap();
           msg!("Counter incremented. Current Count: {}", counter.count);
        Ok(())

    }
}


#[derive(Accounts)]
pub struct Initialize<'info>{
        #[account(init, payer = user, space = 8 + 8)]
        pub counter: Account<'info, Counter>,
        #[account(mut)]
        pub user: Signer<'info>,
        pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info>{
        #[account(mut)]
        pub counter: Account<'info, Counter>,
        pub user: Signer<'info>,

}




#[account]

pub struct Counter {
    pub count: u64,

}

