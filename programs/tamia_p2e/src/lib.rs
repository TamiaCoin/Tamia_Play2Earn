use anchor_lang::prelude::*;

declare_id!("vwgzz5oBpsKtHcySxXk9rjBXvKCAvwg1W1Yh2Ycj1x5");

#[program]
pub mod tamia_p2e {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
