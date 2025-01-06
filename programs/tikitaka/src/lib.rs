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

    fn current_player_index(&self) -> usize {
        ((self.turn - 1) % 2) as usize
    }

    pub fn current_player(&self) -> Pubkey {
        self.players[self.current_player_index()]
    }

    pub fn play(&mut self, tile: &Tile) -> Result<()> {
        require!(self.is_active(), TikitakaError::GameAlreadyOver);

        // First, validate the tile coordinates
        if !(0..=2).contains(&tile.row) || !(0..=2).contains(&tile.column) {
            return Err(TikitakaError::TileOutOfBounds.into());
        }

        // Get indices for cleaner access
        let row = tile.row as usize;
        let col = tile.column as usize;

        // Check if tile is already occupied
        if self.board[row][col].is_some() {
            return Err(TikitakaError::TileAlreadySet.into());
        }

        // Set the tile with current player's sign
        let player_sign =
            Sign::from_usize(self.current_player_index()).expect("Player index should be valid");
        self.board[row][col] = Some(player_sign);

        self.update_state();

        if self.state == GameState::Active {
            self.turn += 1;
        }

        Ok(())
    }

    fn is_winning_trio(&self, trio: [(usize, usize); 3]) -> bool {
        let [first, second, third] = trio;

        self.board[first.0][first.1].is_some()
            && self.board[first.0][first.1] == self.board[second.0][second.1]
            && self.board[first.0][first.1] == self.board[third.0][third.1]
    }
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct Tile {
    row: u8,
    column: u8,
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
