use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

declare_id!("");

#[program]
mod tamia_p2e {
    use super::*;

    pub fn submit_score(ctx: Context<SubmitScore>, score: u64) -> Result<()> {
        let player = &ctx.account.player;
        let game_state = &mut ctx.accounts.game_state;
        let token_mint = &ctx.accounts.token_mint;
        let player_token_account = &ctx.accounts.player_token_account;
        let token_program = &ctx.accounts.token_program;
        msg!("");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SubmitScore {}
