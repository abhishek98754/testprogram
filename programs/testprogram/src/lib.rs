use anchor_lang::prelude::*;

declare_id!("4MJWw1fXkwcxmcvZYgXuAMvLdUaywEF4Y2ZYGkw8f4ZF");

#[program]
pub mod testprogram{
    use super::*;

    pub fn test_function(ctx: Context<TestFunction>) -> Result<()>{
    Ok(())
    }
}

#[derive(Accounts)]
pub struct TestFunction{}