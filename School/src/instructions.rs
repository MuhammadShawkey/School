use crate::context::*;
use crate::errors::ErrorCode;
use crate::events::*;
use crate::utils::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, TransferChecked};

const AMOUNT_REWARD_IN_SOL: u64 = 1000000;
const AMOUNT_REWARD_IN_TOKEN: u64 = 1000000000;

pub fn initialize(ctx: Context<Initialize>, schoolname: String) -> Result<()> {
    ctx.accounts.school.ceo = ctx.accounts.ceo.key();
    require!(schoolname.len() <= 24, ErrorCode::NameTooLong);
    ctx.accounts.school.schoolname = schoolname;
    msg!("School is initialized and its name {:?}", ctx.program_id);
    Ok(())
}

pub fn renameschool(ctx: Context<RenameSchool>, newschoolname: String) -> Result<()> {
    require!(newschoolname.len() <= 24, ErrorCode::NameTooLong);

    ctx.accounts.school.schoolname = newschoolname;

    msg!("School is renamed and its name {:?}", ctx.program_id);
    Ok(())
}

pub fn assignhraddress(ctx: Context<UpdateHr>, hrname: String, hraddress: Pubkey) -> Result<()> {
    require!(hrname.len() <= 24, ErrorCode::NameTooLong);
    require!(
        hrname.chars().all(|c| c.is_ascii_alphabetic()),
        ErrorCode::InvalidCharacters
    );

    ctx.accounts.school.hrname = hrname;
    ctx.accounts.school.hraddress = hraddress;

    msg!(
        "HR '{}' is assigned with address: {}",
        ctx.accounts.school.hrname,
        hraddress
    );
    Ok(())
}

pub fn terminatehraddress(ctx: Context<UpdateHr>) -> Result<()> {
    ctx.accounts.school.hrname = String::new(); // Clear HR name
    ctx.accounts.school.hraddress = Pubkey::default(); // Clear HR address

    msg!(
            "HR is terminated. Cleared hrname and hraddress in the school account #Warning you need to assign new Hr."
        );
    Ok(())
}
pub fn add_student(
    ctx: Context<AddStudent>,
    student_name: String,
    class_index: u8,
    student_pubkey: Pubkey,
) -> Result<()> {
    require!(student_name.len() <= 24, ErrorCode::NameTooLong);
    require!(class_index < 4, ErrorCode::InvalidClassIndex);
    require!(!student_name.is_empty(), ErrorCode::InvalidCharacters);
    for c in student_name.chars() {
        require!(
            c.is_ascii_lowercase() || c.is_ascii_uppercase(),
            ErrorCode::InvalidCharacters
        );
    }

    // Check if the student already exists in any class before proceeding
    let name_exists = ctx
        .accounts
        .students
        .class1_info
        .iter()
        .any(|entry| entry.0 == student_name)
        || ctx
            .accounts
            .students
            .class2_info
            .iter()
            .any(|entry| entry.0 == student_name)
        || ctx
            .accounts
            .students
            .class3_info
            .iter()
            .any(|entry| entry.0 == student_name)
        || ctx
            .accounts
            .students
            .class4_info
            .iter()
            .any(|entry| entry.0 == student_name);

    require!(!name_exists, ErrorCode::StudentAlreadyExists);

    let pubkey_exists = ctx
        .accounts
        .students
        .class1_info
        .iter()
        .any(|entry| entry.1 == student_pubkey)
        || ctx
            .accounts
            .students
            .class2_info
            .iter()
            .any(|entry| entry.1 == student_pubkey)
        || ctx
            .accounts
            .students
            .class3_info
            .iter()
            .any(|entry| entry.1 == student_pubkey)
        || ctx
            .accounts
            .students
            .class4_info
            .iter()
            .any(|entry| entry.1 == student_pubkey);

    require!(!pubkey_exists, ErrorCode::StudentAlreadyExists);

    match class_index {
        0 => {
            require!(
                ctx.accounts.students.class1_info.len() < 5,
                ErrorCode::ClassFull
            );
            ctx.accounts
                .students
                .class1_info
                .push((student_name.clone(), student_pubkey, 0, 0));
        }
        1 => {
            require!(
                ctx.accounts.students.class2_info.len() < 5,
                ErrorCode::ClassFull
            );
            ctx.accounts
                .students
                .class2_info
                .push((student_name.clone(), student_pubkey, 0, 0));
        }
        2 => {
            require!(
                ctx.accounts.students.class3_info.len() < 5,
                ErrorCode::ClassFull
            );
            ctx.accounts
                .students
                .class3_info
                .push((student_name.clone(), student_pubkey, 0, 0));
        }
        3 => {
            require!(
                ctx.accounts.students.class4_info.len() < 5,
                ErrorCode::ClassFull
            );
            ctx.accounts
                .students
                .class4_info
                .push((student_name.clone(), student_pubkey, 0, 0));
        }
        _ => return err!(ErrorCode::InvalidClassIndex),
    }

    msg!(
        "Added student {} with pubkey {} to class {}",
        student_name,
        student_pubkey,
        class_index
    );
    ctx.accounts.school.totalstudents += 1;

    Ok(())
}

pub fn fundtreasurynative(ctx: Context<FundTreasuryNative>, amount_in_lamports: u64) -> Result<()> {
    let cpi_accounts_native = anchor_lang::system_program::Transfer {
        from: ctx.accounts.signer.to_account_info(),
        to: ctx.accounts.treasurydash.to_account_info(),
    };
    let cpi_context = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        cpi_accounts_native,
    );
    anchor_lang::system_program::transfer(cpi_context, amount_in_lamports)?;
    Ok(())
}

pub fn fundtreasurytokens(ctx: Context<FundTreasuryTokens>, amount_token_fund: u64) -> Result<()> {
    let cpi_accounts = Transfer {
        from: ctx.accounts.senderata.to_account_info(),
        to: ctx
            .accounts
            .treasury_token_account_reciever
            .to_account_info(),
        authority: ctx.accounts.owneraddress.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_ctx, amount_token_fund)?;

    Ok(())
}

pub fn evaluate_student(
    ctx: Context<EvaluateStudent>,
    student_name: String,
    class_index: u8,
    score: i64,
    reason: String,
) -> Result<()> {
    checkname(&ctx, student_name.clone()).unwrap();
    let mut students = ctx.accounts.students.clone();
    require!(reason.len() <= 24, ErrorCode::ReasonTooLong);

    // Isolate the target class vector based on the class index
    let class_scores: *mut Vec<(String, Pubkey, i64, u8)> = match class_index {
        0 => &mut students.class1_info,
        1 => &mut students.class2_info,
        2 => &mut students.class3_info,
        3 => &mut students.class4_info,
        _ => return err!(ErrorCode::InvalidClassIndex),
    };

    let mut _deserve_one_more_reward_in_sol = false;
    // Mutably update the student's record
    let total_score: i64 = unsafe {
        let class_scores = &mut *class_scores;
        let student_entry = class_scores
            .iter_mut()
            .find(|entry| entry.0 == student_name);

        match student_entry {
            Some(entry) => {
                // Reward flag
                if entry.2 % 100 + score % 100 >= 100 {
                    _deserve_one_more_reward_in_sol = true;
                } else {
                    _deserve_one_more_reward_in_sol = false;
                }
                entry.2 += score;
                entry.3 = (entry.2 / 500) as u8;
                entry.2
            }
            None => return err!(ErrorCode::StudentNotFound),
        }
    };

    // Emit evaluation event
    emit!(StudentEvaluated {
        student_name: student_name.clone(),
        class_index,
        score,
        reason: reason.clone(),
        total_score,
    });

    // Check for termination
    if total_score <= -100 {
        terminate_student(&mut **students, &student_name, class_index)?;
        msg!(
            "Student '{}' terminated for reaching a total score of {}",
            student_name,
            total_score
        );
        emit!(StudentTerminated {
            student_name: student_name.clone(),
            class_index,
            total_score,
        });
    }

    // Reward logic
    if score >= 100 {
        send_reward_native(_deserve_one_more_reward_in_sol, score as u64, &ctx)?;
        msg!(
            "Transferred {} lamports to student '{}' for reaching a total score of {}",
            AMOUNT_REWARD_IN_SOL,
            student_name,
            total_score
        );
        emit!(StudentRewarded {
            student_name: student_name.clone(),
            reward: AMOUNT_REWARD_IN_SOL,
        });
    }

    ctx.accounts.students = students;

    msg!(
        "Student '{}' evaluated with score {} for reason '{}'",
        student_name,
        score,
        reason
    );

    Ok(())
}

pub fn claimrewardtoken(
    ctx: Context<ClaimRewardToken>,
    student_name: String,
    class_index: u8,
) -> Result<()> {
    // Check if the class index is valid (0 to 3)
    require!(class_index <= 3, ErrorCode::InvalidClassIndex);

    let students = &mut ctx.accounts.students; // Access the students account directly
    let student_entry = match class_index {
        0 => students
            .class1_info
            .iter_mut()
            .find(|entry| entry.0 == student_name),
        1 => students
            .class2_info
            .iter_mut()
            .find(|entry| entry.0 == student_name),
        2 => students
            .class3_info
            .iter_mut()
            .find(|entry| entry.0 == student_name),
        3 => students
            .class4_info
            .iter_mut()
            .find(|entry| entry.0 == student_name),
        _ => return err!(ErrorCode::InvalidClassIndex), // If class_index is invalid
    };

    // Check if the student exists
    let student_entry = match student_entry {
        Some(entry) => entry,
        None => return err!(ErrorCode::StudentNotFound),
    };  

    let token_deserved = AMOUNT_REWARD_IN_TOKEN * student_entry.3 as u64;

    // Transfer the reward token to the student's account
    let cpi_accounts =  TransferChecked {
        from: ctx.accounts.treasury_token_account.to_account_info(),
        to: ctx.accounts.signer_token_account.to_account_info(),
        authority: ctx.accounts.treasurydash.to_account_info(),
        mint: ctx.accounts.token_mint.to_account_info(),
    };
    let seeds = &["treasurydash".as_bytes(), &[ctx.bumps.treasurydash]];
    let seeds_account = [&seeds[..]];

    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new_with_signer(cpi_program,
         cpi_accounts,
         &seeds_account);
    token::transfer_checked(cpi_ctx, token_deserved.try_into().unwrap(),ctx.accounts.token_mint.decimals)?;
    student_entry.2=0;
    student_entry.3 = 0;    

    msg!(
        "Successfully transferred {} tokens to student '{}' from class {}",
        token_deserved,
        student_name,
        class_index
    );

    Ok(())
}
