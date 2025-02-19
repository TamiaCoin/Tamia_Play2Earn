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
pub struct SubmitScore<'info> {
    #[account(mut)]
    pub player: Signer<'info>,

    #[account(mut)]
    pub game_state: Account<'info, GameState>,
    
    #[account(mut)]
    pub player_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub treasury_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

#[account]
pub struct GameState {
    pub best_score: u64,
    pub treasury_token_account: Pubkey,
}

#[error_code]
pub enum Play2EarnError {
    #[msg("The submitted score is invalid")]
    InvalidScore,
    #[msg("No reward for this score")]
    NoReward,
}

fn calculate_reward(score: u64) -> u64 {
    match score {
        0..=100 => 0,
        101..=200 => score * 10_000,
        201..=400 => score * 50_000,
        401..=999 => score * 100_000,
        1000..=u64::MAX => score * 200_000 * 2, // Jackpot x2 :)
    }
}