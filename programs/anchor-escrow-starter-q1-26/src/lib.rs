use anchor_lang::prelude::*;
pub mod state;
pub use state::*;
pub mod instructions;
pub use instructions::*;
declare_id!("93UWPgTx9eMd8rASGvEsfXcAdphAuSnPcSzTpasdruUm");

#[program]
pub mod anchor_escrow_starter_q1_26 {
    use super::*;

    pub fn make(ctx: Context<Make>, seeds: u64, deposit: u64, receive: u64) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        ctx.accounts.init_escrow(seeds, receive, &ctx.bumps)?;
        ctx.accounts.deposit(deposit)
        // Ok(())
    }
    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        ctx.accounts.refund_and_close_vault()
    }
    pub fn take(ctx: Context<Take>) -> Result<()> {
        ctx.accounts.maker_transfer()?;
        ctx.accounts.vault_transfer()?;
        ctx.accounts.close_vault()
    }
}

#[derive(Accounts)]
pub struct Initialize {}
