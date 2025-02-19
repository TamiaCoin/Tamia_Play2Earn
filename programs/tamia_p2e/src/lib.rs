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

        // Check that the score is higher than the player's old score
        require!(score > game_stable.best_score, Play2EarnError::InvalidScore);
        game_state.best_score = score;

        // Calculate the reward
        let reward = calculate_reward(score);
        require!(reward > 0, Play2EarnError::NoReward);

        // Transfer tokens at player

        msg!("");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SubmitScore<'info>
