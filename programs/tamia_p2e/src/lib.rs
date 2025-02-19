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
        let cpi_accounts = Transfer {
            from: game_state.treasury_token_account.to_account_info(),
            to: player_token_account.to_account_info(),
            authority: game_state.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(token_program.to_account_info(), cpi_accounts);
        token::transfer(cpi_ctx, reward)?;

        msg!("Score validated: {} | Reward: {} tokens", score, reward);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SubmitScore<'info>{}
