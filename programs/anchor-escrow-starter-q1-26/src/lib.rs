use anchor_lang::prelude::*;

declare_id!("93UWPgTx9eMd8rASGvEsfXcAdphAuSnPcSzTpasdruUm");

#[program]
pub mod anchor_escrow_starter_q1_26 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
