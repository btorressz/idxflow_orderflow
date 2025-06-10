use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

declare_id!("BH4AjERjFgoXUqJFN8GkuF1rgWQzidBwBkiibsHHpVJR");

#[program]
pub mod idxflow_orderflow {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        reward_rate_per_volume: u64, // Tokens per dollar of volume
        epoch_duration: i64,         // Duration in seconds
        min_volume_threshold: u64,   // Minimum volume to be eligible for rewards
    ) -> Result<()> {
        let global_state = &mut ctx.accounts.global_state;
        global_state.authority = ctx.accounts.authority.key();
        global_state.reward_rate_per_volume = reward_rate_per_volume;
        global_state.epoch_duration = epoch_duration;
        global_state.min_volume_threshold = min_volume_threshold;
        global_state.current_epoch = 0;
        global_state.last_epoch_reset = Clock::get()?.unix_timestamp;
        global_state.total_distributed = 0;
        global_state.bump = ctx.bumps.global_state;
        
        Ok(())
    }

    pub fn create_user_account(ctx: Context<CreateUserAccount>) -> Result<()> {
        let user_account = &mut ctx.accounts.user_account;
        user_account.owner = ctx.accounts.user.key();
        user_account.total_volume = 0;
        user_account.current_epoch_volume = 0;
        user_account.staked_amount = 0;
        user_account.fee_tier = FeeTier::Bronze;
        user_account.last_claim_epoch = 0;
        user_account.bump = ctx.bumps.user_account;
        
        Ok(())
    }

    pub fn record_swap_volume(
        ctx: Context<RecordSwapVolume>,
        volume: u64,
    ) -> Result<()> {
        let global_state = &mut ctx.accounts.global_state;
        let user_account = &mut ctx.accounts.user_account;
        let current_time = Clock::get()?.unix_timestamp;

        // Check if we need to start a new epoch
        if current_time >= global_state.last_epoch_reset + global_state.epoch_duration {
            global_state.current_epoch += 1;
            global_state.last_epoch_reset = current_time;
            
            // Reset current epoch volume for all users (handled per user basis)
            user_account.current_epoch_volume = 0;
        }

        // Update volume tracking
        user_account.total_volume = user_account.total_volume.checked_add(volume).unwrap();
        user_account.current_epoch_volume = user_account.current_epoch_volume.checked_add(volume).unwrap();

        msg!("Recorded {} volume for user {}", volume, user_account.owner);
        
        Ok(())
    }

    pub fn stake_tokens(
        ctx: Context<StakeTokens>,
        amount: u64,
    ) -> Result<()> {
        let user_account = &mut ctx.accounts.user_account;
        
        // Transfer tokens from user to staking vault
        let transfer_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.user_token_account.to_account_info(),
                to: ctx.accounts.staking_vault.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        );
        token::transfer(transfer_ctx, amount)?;

        // Update staked amount and fee tier
        user_account.staked_amount = user_account.staked_amount.checked_add(amount).unwrap();
        user_account.fee_tier = calculate_fee_tier(user_account.staked_amount);

        msg!("Staked {} tokens, new tier: {:?}", amount, user_account.fee_tier);
        
        Ok(())
    }

    pub fn unstake_tokens(
        ctx: Context<UnstakeTokens>,
        amount: u64,
    ) -> Result<()> {
        let user_account = &mut ctx.accounts.user_account;
        
        require!(user_account.staked_amount >= amount, ErrorCode::InsufficientStakedAmount);

        // Transfer tokens from staking vault back to user
        let seeds = &[
            b"global_state".as_ref(),
            &[ctx.accounts.global_state.bump],
        ];
        let signer = &[&seeds[..]];
        
        let transfer_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.staking_vault.to_account_info(),
                to: ctx.accounts.user_token_account.to_account_info(),
                authority: ctx.accounts.global_state.to_account_info(),
            },
            signer,
        );
        token::transfer(transfer_ctx, amount)?;

        // Update staked amount and fee tier
        user_account.staked_amount = user_account.staked_amount.checked_sub(amount).unwrap();
        user_account.fee_tier = calculate_fee_tier(user_account.staked_amount);

        msg!("Unstaked {} tokens, new tier: {:?}", amount, user_account.fee_tier);
        
        Ok(())
    }

    pub fn claim_rewards(ctx: Context<ClaimRewards>) -> Result<()> {
        let global_state = &mut ctx.accounts.global_state;
        let user_account = &mut ctx.accounts.user_account;
        let current_time = Clock::get()?.unix_timestamp;

        // Ensure user hasn't claimed for current epoch
        require!(
            user_account.last_claim_epoch < global_state.current_epoch,
            ErrorCode::AlreadyClaimedThisEpoch
        );

        // Check if user meets minimum volume threshold
        require!(
            user_account.current_epoch_volume >= global_state.min_volume_threshold,
            ErrorCode::InsufficientVolume
        );

        // Calculate reward amount
        let reward_amount = user_account.current_epoch_volume
            .checked_mul(global_state.reward_rate_per_volume)
            .unwrap()
            .checked_div(1_000_000) // Assuming 6 decimal places for precision
            .unwrap();

        // Apply staking multiplier based on fee tier
        let multiplier = get_reward_multiplier(user_account.fee_tier);
        let final_reward = reward_amount
            .checked_mul(multiplier)
            .unwrap()
            .checked_div(100) // Multiplier is in basis points
            .unwrap();

        // Transfer reward tokens to user
        let seeds = &[
            b"global_state".as_ref(),
            &[global_state.bump],
        ];
        let signer = &[&seeds[..]];
        
        let transfer_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.reward_vault.to_account_info(),
                to: ctx.accounts.user_token_account.to_account_info(),
                authority: global_state.to_account_info(),
            },
            signer,
        );
        token::transfer(transfer_ctx, final_reward)?;

        // Update claim tracking
        user_account.last_claim_epoch = global_state.current_epoch;
        global_state.total_distributed = global_state.total_distributed.checked_add(final_reward).unwrap();

        msg!("Claimed {} reward tokens for user {}", final_reward, user_account.owner);
        
        Ok(())
    }

    pub fn get_fee_discount(ctx: Context<GetFeeDiscount>) -> Result<u16> {
        let user_account = &ctx.accounts.user_account;
        let discount = match user_account.fee_tier {
            FeeTier::Bronze => 0,   // 0% discount
            FeeTier::Silver => 10,  // 10% discount
            FeeTier::Gold => 25,    // 25% discount
            FeeTier::Platinum => 50, // 50% discount
            FeeTier::Diamond => 75,  // 75% discount
        };
        
        Ok(discount)
    }

    pub fn update_reward_rate(
        ctx: Context<UpdateRewardRate>,
        new_rate: u64,
    ) -> Result<()> {
        let global_state = &mut ctx.accounts.global_state;
        require!(
            ctx.accounts.authority.key() == global_state.authority,
            ErrorCode::Unauthorized
        );
        
        global_state.reward_rate_per_volume = new_rate;
        msg!("Updated reward rate to {}", new_rate);
        
        Ok(())
    }
}

// Account Structs
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = GlobalState::SIZE,
        seeds = [b"global_state"],
        bump
    )]
    pub global_state: Account<'info, GlobalState>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateUserAccount<'info> {
    #[account(
        init,
        payer = user,
        space = UserAccount::SIZE,
        seeds = [b"user_account", user.key().as_ref()],
        bump
    )]
    pub user_account: Account<'info, UserAccount>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RecordSwapVolume<'info> {
    #[account(mut)]
    pub global_state: Account<'info, GlobalState>,
    
    #[account(
        mut,
        seeds = [b"user_account", user.key().as_ref()],
        bump = user_account.bump
    )]
    pub user_account: Account<'info, UserAccount>,
    
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct StakeTokens<'info> {
    #[account(
        mut,
        seeds = [b"user_account", user.key().as_ref()],
        bump = user_account.bump
    )]
    pub user_account: Account<'info, UserAccount>,
    
    #[account(mut)]
    pub global_state: Account<'info, GlobalState>,
    
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub staking_vault: Account<'info, TokenAccount>,
    
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct UnstakeTokens<'info> {
    #[account(
        mut,
        seeds = [b"user_account", user.key().as_ref()],
        bump = user_account.bump
    )]
    pub user_account: Account<'info, UserAccount>,
    
    #[account(
        mut,
        seeds = [b"global_state"],
        bump = global_state.bump
    )]
    pub global_state: Account<'info, GlobalState>,
    
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub staking_vault: Account<'info, TokenAccount>,
    
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct ClaimRewards<'info> {
    #[account(mut)]
    pub global_state: Account<'info, GlobalState>,
    
    #[account(
        mut,
        seeds = [b"user_account", user.key().as_ref()],
        bump = user_account.bump
    )]
    pub user_account: Account<'info, UserAccount>,
    
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub reward_vault: Account<'info, TokenAccount>,
    
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct GetFeeDiscount<'info> {
    #[account(
        seeds = [b"user_account", user.key().as_ref()],
        bump = user_account.bump
    )]
    pub user_account: Account<'info, UserAccount>,
    
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateRewardRate<'info> {
    #[account(
        mut,
        seeds = [b"global_state"],
        bump = global_state.bump
    )]
    pub global_state: Account<'info, GlobalState>,
    
    pub authority: Signer<'info>,
}

// State Structs
#[account]
pub struct GlobalState {
    pub authority: Pubkey,
    pub reward_rate_per_volume: u64,
    pub epoch_duration: i64,
    pub min_volume_threshold: u64,
    pub current_epoch: u64,
    pub last_epoch_reset: i64,
    pub total_distributed: u64,
    pub bump: u8,
}

impl GlobalState {
    pub const SIZE: usize = 8 + 32 + 8 + 8 + 8 + 8 + 8 + 8 + 1;
}

#[account]
pub struct UserAccount {
    pub owner: Pubkey,
    pub total_volume: u64,
    pub current_epoch_volume: u64,
    pub staked_amount: u64,
    pub fee_tier: FeeTier,
    pub last_claim_epoch: u64,
    pub bump: u8,
}

impl UserAccount {
    pub const SIZE: usize = 8 + 32 + 8 + 8 + 8 + 1 + 8 + 1;
}

// Enums
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum FeeTier {
    Bronze,   // 0 - 999 tokens
    Silver,   // 1,000 - 9,999 tokens
    Gold,     // 10,000 - 49,999 tokens
    Platinum, // 50,000 - 99,999 tokens
    Diamond,  // 100,000+ tokens
}

// Helper Functions
fn calculate_fee_tier(staked_amount: u64) -> FeeTier {
    match staked_amount {
        0..=999_000_000 => FeeTier::Bronze,        // 0-999 tokens (assuming 6 decimals)
        1_000_000_000..=9_999_000_000 => FeeTier::Silver,    // 1K-9.999K tokens
        10_000_000_000..=49_999_000_000 => FeeTier::Gold,    // 10K-49.999K tokens
        50_000_000_000..=99_999_000_000 => FeeTier::Platinum, // 50K-99.999K tokens
        _ => FeeTier::Diamond,                     // 100K+ tokens
    }
}

fn get_reward_multiplier(tier: FeeTier) -> u64 {
    match tier {
        FeeTier::Bronze => 100,   // 1.0x multiplier (100%)
        FeeTier::Silver => 125,   // 1.25x multiplier
        FeeTier::Gold => 150,     // 1.5x multiplier
        FeeTier::Platinum => 200, // 2.0x multiplier
        FeeTier::Diamond => 300,  // 3.0x multiplier
    }
}

// Error Codes
#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient staked amount")]
    InsufficientStakedAmount,
    #[msg("Already claimed rewards for this epoch")]
    AlreadyClaimedThisEpoch,
    #[msg("Insufficient volume to claim rewards")]
    InsufficientVolume,
    #[msg("Unauthorized")]
    Unauthorized,
}