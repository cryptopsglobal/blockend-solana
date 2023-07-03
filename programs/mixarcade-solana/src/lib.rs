use anchor_lang::prelude::*;

declare_id!("5kDHrYTjDZ19d8jdj1RoVjomjLoAYK2XQPeBfZdFFMFL");

#[program]
pub mod mixarcade_solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
