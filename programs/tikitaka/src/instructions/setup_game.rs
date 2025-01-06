use crate::state::game::*;
use anchor_lang::prelude::*;

pub fn setup_game(ctx: Context<SetupGame>, player_two: Pubkey) -> Result<()> {
    ctx.accounts
        .game
        .start([ctx.accounts.player_one.key(), player_two])
}

#[derive(Accounts)]
pub struct SetupGame<'info> {
    #[account(mut)]
    pub player_one: Signer<'info>,
    // init creates rent-exempt accounts
    #[account(init, payer = player_one, space = 8 + Game::MAXIMUM_SIZE)]
    pub game: Account<'info, Game>,
    pub system_program: Program<'info, System>,
}
