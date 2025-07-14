use anchor_lang::prelude::*;

pub mod context;
pub mod errors;
pub mod events;
pub mod instructions;
pub mod structs;
pub mod utils;
use crate::context::*;

declare_id!("zKDU6TVerMNPx4eVrFZutFBb4CWD6DzauTtYohjJobW");

#[program]
pub mod elmadrasa {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, schoolname: String) -> Result<()> {
        instructions::initialize(ctx, schoolname)
    }

    pub fn renameschool(ctx: Context<RenameSchool>, newschoolname: String) -> Result<()> {
        instructions::renameschool(ctx, newschoolname)
    }

    pub fn assignhraddress(
        ctx: Context<UpdateHr>,
        hrname: String,
        hraddress: Pubkey,
    ) -> Result<()> {
        instructions::assignhraddress(ctx, hrname, hraddress)
    }

    pub fn terminatehraddress(ctx: Context<UpdateHr>) -> Result<()> {
        instructions::terminatehraddress(ctx)
    }
    pub fn add_student(
        ctx: Context<AddStudent>,
        student_name: String,
        class_index: u8,
        student_pubkey: Pubkey,
    ) -> Result<()> {
        instructions::add_student(ctx, student_name, class_index, student_pubkey)
    }

    pub fn fundtreasurynative(
        ctx: Context<FundTreasuryNative>,
        amount_in_lamports: u64,
    ) -> Result<()> {
        instructions::fundtreasurynative(ctx, amount_in_lamports)
    }

    pub fn fundtreasurytokens(
        ctx: Context<FundTreasuryTokens>,
        amount_token_fund: u64,
    ) -> Result<()> {
        instructions::fundtreasurytokens(ctx, amount_token_fund)
    }

    pub fn evaluate_student(
        ctx: Context<EvaluateStudent>,
        student_name: String,
        class_index: u8,
        score: i64,
        reason: String,
    ) -> Result<()> {
        instructions::evaluate_student(ctx, student_name, class_index, score, reason)
    }

    pub fn claimrewardtoken(
        ctx: Context<ClaimRewardToken>,
        student_name: String,
        class_index: u8,
    ) -> Result<()> {
        instructions::claimrewardtoken(ctx, student_name, class_index)
    }
}
