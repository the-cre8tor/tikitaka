use anchor_lang::prelude::*;
use num_derive::*;
use num_traits::*;

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

#[account]
pub struct Game {
    players: [Pubkey; 2],
    turn: u8,
    board: [[Option<Sign>; 3]; 3],
    state: GameState,
}

impl Game {
    const PLAYERS_SIZE: usize = 32 * 2;
    const TURN_SIZE: usize = 1;
    const BOARD_SIZE: usize = 9 * (1 + 1);
    const STATE_SIZE: usize = 33;

    pub const MAXIMUM_SIZE: usize =
        Self::PLAYERS_SIZE + Self::TURN_SIZE + Self::BOARD_SIZE + Self::STATE_SIZE;

    pub fn start(&mut self, players: [Pubkey; 2]) -> Result<()> {
        require_eq!(self.turn, 0, TikitakaError::GameAlreadyStarted);

        self.players = players;
        self.turn = 1;

        Ok(())
    }

    pub fn is_active(&self) -> bool {
        self.state == GameState::Active
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum GameState {
    Active,
    Tie,
    Won { winner: Pubkey },
}

#[derive(
    AnchorSerialize, AnchorDeserialize, FromPrimitive, ToPrimitive, Clone, Copy, PartialEq, Eq,
)]
pub enum Sign {
    X,
    O,
}

#[error_code]
pub enum TikitakaError {
    TileOutOfBounds,
    TileAlreadySet,
    GameAlreadyOver,
    NotPlayersTurn,
    GameAlreadyStarted,
}
