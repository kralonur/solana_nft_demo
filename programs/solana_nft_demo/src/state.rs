use anchor_lang::prelude::*;

#[account]
pub struct ContractData {
    pub bump: u8,
    pub treasury_bump: u8,
    pub authority: Pubkey,
    pub fee: u64,
    pub total_minted: u16, // 65536
}
impl ContractData {
    pub const SPACE: usize = 1 + 1 + 32 + 8 + 2;
    pub const SEED: &'static [u8] = b"contractdata";
    pub const LIMIT: u16 = 3; // MAX mint limit
}

pub const TREASURY_SEED: &[u8] = b"treasury";
