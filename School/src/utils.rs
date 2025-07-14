use crate::context::*;
use crate::errors::ErrorCode;
use crate::structs::Students;
use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer as NativeTransfer};

const AMOUNT_REWARD_IN_SOL: u64 = 1000000;

pub fn terminate_student(
    students: &mut Students,
    student_name: &String,
    class_index: u8,
) -> Result<()> {
    require!((class_index as usize) < 4, ErrorCode::InvalidClassIndex);

    match class_index {
        0 => students
            .class1_info
            .retain(|entry| &entry.0 != student_name),
        1 => students
            .class2_info
            .retain(|entry| &entry.0 != student_name),
        2 => students
            .class3_info
            .retain(|entry| &entry.0 != student_name),
        3 => students
            .class4_info
            .retain(|entry| &entry.0 != student_name),
        _ => return err!(ErrorCode::StudentNotFound),
    }

    msg!(
        "Student {} has been terminated from class {}",
        student_name,
        class_index
    );

    Ok(())
}

pub fn send_reward_native<'info>(
    _deserve_one_more_reward_in_sol: bool,
    score: u64,
    ctx: &Context<EvaluateStudent>,
) -> Result<()> {
    // Uses the globally defined constant
    // const AMOUNT_REWARD_IN_SOL: u64 = 1_000_000; should be declared at the top of your program

    let mut prize: u64 = score / 100 * AMOUNT_REWARD_IN_SOL;

    if _deserve_one_more_reward_in_sol {
        prize += AMOUNT_REWARD_IN_SOL;
    }

    let seeds = &["treasurydash".as_bytes(), &[ctx.bumps.treasurydash]];
    let seeds_account = [&seeds[..]];

    let cpi_context_reward = CpiContext::new_with_signer(
        ctx.accounts.system_program.to_account_info(),
        NativeTransfer {
            from: ctx.accounts.treasurydash.to_account_info(),
            to: ctx.accounts.receipient_account.to_account_info(),
        },
        &seeds_account,
    );

    transfer(cpi_context_reward, prize)?;

    msg!("Sent {} lamports as reward based on score {}", prize, score);

    Ok(())
}

pub fn checkname<'info>(ctx: &Context<EvaluateStudent>, student_name: String) -> Result<()> {
    let students = &ctx.accounts.students;

    // Check if the student exists in any class
    let name_exists = students
        .class1_info
        .iter()
        .any(|entry| entry.0 == student_name)
        || students
            .class2_info
            .iter()
            .any(|entry| entry.0 == student_name)
        || students
            .class3_info
            .iter()
            .any(|entry| entry.0 == student_name)
        || students
            .class4_info
            .iter()
            .any(|entry| entry.0 == student_name);
    require!(name_exists, ErrorCode::StudentNotFound);
    Ok(())
}
