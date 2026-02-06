#![allow(unexpected_cfgs)]
#![allow(deprecated)]

use anchor_lang::prelude::*;

mod error;
mod state;
mod instructions;
mod tests;

use instructions::*;
use error::*;

declare_id!("FircrADQ2wgGuvpm8qneNCfKM7o5zoHTWnDQxngpTQ3J");

#[program]
pub mod anchor_escrow {
    use super::*;

    pub fn make(ctx: Context<Make>, seed: u64, deposit: u64, receive: u64, is_time_lock_required: bool) -> Result<()> {
        ctx.accounts.init_escrow(seed, receive, is_time_lock_required, &ctx.bumps)?;
        ctx.accounts.deposit(deposit)
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        ctx.accounts.refund_and_close_vault()
    }

    pub fn take(ctx: Context<Take>) -> Result<()> {
        if ctx.accounts.escrow.is_time_lock_required {
            let current_timestamp = Clock::get()?.unix_timestamp;
            let time_elapsed = current_timestamp - ctx.accounts.escrow.initial_timestamp;

            require!(
                time_elapsed >= 5 * 24 * 60 * 60, // 5 days in seconds
                EscrowError::EscrowStillLocked
            );
        }
        
        ctx.accounts.deposit()?;
        ctx.accounts.withdraw_and_close_vault()
    }
}