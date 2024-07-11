use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Index out of bounds.")]
    IndexOutOfBounds,
    #[msg("Impossible move.")]
    MoveImpossible,
    #[msg("Unknown direction.")]
    UnknownDirection,
    #[msg("Wrong data.")]
    InitialisationFailed,
}