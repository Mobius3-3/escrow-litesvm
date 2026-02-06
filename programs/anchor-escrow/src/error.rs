use anchor_lang::prelude::*;

#[error_code]
pub enum EscrowError {
    #[msg("Escrow is still locked. Must wait 5 days after creation.")]
    EscrowStillLocked,
}
