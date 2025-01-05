use anchor_lang::prelude::*;

declare_id!("H3g2QDsGtNtAcfuHDA6oeGdeq2w3LjxNrytS7FuwLFB5");

#[program]
pub mod tikitaka {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
