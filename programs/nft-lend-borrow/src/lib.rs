use anchor_lang::prelude::*;
pub mod states;
pub use states::*;


declare_id!("AR7FastXfwZX2q1KsEhWwdFZJqhzr8dFqUvwBhAs1s77");

#[program]
pub mod nft_lend_borrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
