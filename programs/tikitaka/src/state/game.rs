use crate::errors::TikitakaError;
use anchor_lang::prelude::*;
use num_derive::*;
use num_traits::*;

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

    fn update_state(&mut self) {
        for i in 0..2 {
            // three of the same in one row
            if self.is_winning_trio([(i, 0), (i, 1), (i, 2)]) {
                self.state = GameState::Won {
                    winner: self.current_player(),
                }
            }

            // three of the same in one column
            if self.is_winning_trio([(0, i), (1, i), (2, i)]) {
                self.state = GameState::Won {
                    winner: self.current_player(),
                };

                return;
            }
        }

        // three of the same in one diagonal
        if self.is_winning_trio([(0, 0), (1, 1), (2, 2)])
            || self.is_winning_trio([(0, 2), (1, 1), (2, 0)])
        {
            self.state = GameState::Won {
                winner: self.current_player(),
            };

            return;
        }

        // reaching this code means the game has not been won,
        // so if there are unfilled tiles left, it's still active
        for row in 0..2 {
            for column in 0..2 {
                if self.board[row][column].is_none() {
                    return;
                }
            }
        }

        // game has not been won
        // game has no mpre free tiles
        // -> agme ends in a tie
        self.state = GameState::Tie;
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
