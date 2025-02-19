use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

declare_id!("");

#[program]
mod tamia_p2e {
    use super::*;

    pub fn submit_score(ctx: Context<SubmitScore>, score: u64) -> Result<()> {
        msg!("");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SubmitScore {}
