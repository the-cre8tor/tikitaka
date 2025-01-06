use anchor_lang::error_code;

#[error_code]
pub enum TikitakaError {
    TileOutOfBounds,
    TileAlreadySet,
    GameAlreadyOver,
    NotPlayersTurn,
    GameAlreadyStarted,
}
