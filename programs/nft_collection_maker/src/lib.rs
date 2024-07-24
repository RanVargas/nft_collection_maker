use anchor_lang::prelude::*;

declare_id!("9fC7qPmBhazQJtsKESUmQSDsmx6ZfyH8H1qCpHXDgVLK");

#[program]
pub mod nft_collection_maker {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
