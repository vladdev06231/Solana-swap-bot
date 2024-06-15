use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("You are not authorized to perform this action.")]
    Unauthorized,

    #[msg("You are not authorized to perform this action.")]
    NotAllowedAuthority,

    #[msg("ZeroAddressDetected")]
    ZeroAddressDetected,
}
