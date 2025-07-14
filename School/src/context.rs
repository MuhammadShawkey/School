use crate::errors::ErrorCode;
use crate::structs::School;
use crate::structs::Students;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut, signer)]
    pub ceo: Signer<'info>,

    #[account(
        init,
        payer = ceo,
        space = School::LEN,
        seeds = [b"school"],
        bump
    )]
    pub school: Box<Account<'info, School>>,
    pub system_program: Program<'info, System>,

    #[account(
        init,
        payer = ceo,
        space = Students::LEN,
        seeds = [b"students"],
        bump
    )]
    pub students: Box<Account<'info, Students>>,
}

#[derive(Accounts)]
pub struct RenameSchool<'info> {
    #[account(mut, constraint = signer.key() == school.ceo @ ErrorCode::UnAuthorized)]
    pub signer: Signer<'info>,
    pub school: Box<Account<'info, School>>,
}

#[derive(Accounts)]
pub struct UpdateHr<'info> {
    #[account(mut, constraint = signer.key() == school.ceo @ ErrorCode::UnAuthorized)]
    pub signer: Signer<'info>,

    #[account(mut)]
    /// CHECK: The school account is initialized with known structure and will be used safely.
    pub school: Box<Account<'info, School>>, // This is a checked account, and it's safe to use
}

#[derive(Accounts)]
pub struct AddStudent<'info> {
    #[account(mut, constraint = signer.key() == school.hraddress @ ErrorCode::UnAuthorized)]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub school: Box<Account<'info, School>>,
    #[account(mut)]
    pub students: Box<Account<'info, Students>>,
}

#[derive(Accounts)]
pub struct EvaluateStudent<'info> {
    #[account(mut, constraint = signer.key() == school.hraddress @ ErrorCode::UnAuthorized)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub school: Box<Account<'info, School>>,

    #[account(mut)]
    pub students: Box<Account<'info, Students>>,

    #[account(mut)]
    pub receipient_account: SystemAccount<'info>,

    #[account(
        mut,
        seeds = [b"treasurydash"],
        bump,
    )]
    pub treasurydash: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct FundTreasuryNative<'info> {
    #[account(mut, signer)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [b"treasurydash"],
        bump,
    )]
    pub treasurydash: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct FundTreasuryTokens<'info> {
    #[account(mut, signer)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub senderata: Account<'info, TokenAccount>,

    #[account(mut)]
    pub treasury_token_account_reciever: Account<'info, TokenAccount>,
    #[account(mut)]
    pub owneraddress: SystemAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub token_mint: Account<'info, Mint>,
}

#[derive(Accounts)]
pub struct ClaimRewardToken<'info> {
    #[account(mut, signer)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub signer_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"treasurydash"],
        bump,
    )]
    pub treasurydash: SystemAccount<'info>,
    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = treasurydash
    )]
    pub treasury_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub token_mint: Account<'info, Mint>,
    
    #[account(mut)]
    pub students: Box<Account<'info, Students>>,
}
