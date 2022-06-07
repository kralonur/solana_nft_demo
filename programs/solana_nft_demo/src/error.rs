use anchor_lang::prelude::*;

#[error_code]
pub enum CustomErrors {
    // 6000 0x1770
    #[msg("SOLs is not enough")]
    RequestRentSol,
    // 6001 0x1771
    #[msg("SOLs is not enough")]
    InsufficientFunds,
}
