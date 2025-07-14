use anchor_lang::prelude::*;

#[account]
pub struct School {
    pub ceo: Pubkey,
    pub schoolname: String,
    pub hrname: String,
    pub hraddress: Pubkey,
    pub totalstudents: u8,
}

impl School {
    pub const LEN: usize = 8 + 32 + 24 + 24 + 32 + 1; // Account length with admin field (8 for base size + 32 for Pubkey)
}

#[account]
pub struct Students {
    pub class1_info: Vec<(String, Pubkey, i64, u8)>, // (student_name,student_pubkey total_score , number of deserved tokens )
    pub class2_info: Vec<(String, Pubkey, i64, u8)>, // (student_name,student_pubkey total_score , number of deserved tokens)
    pub class3_info: Vec<(String, Pubkey, i64, u8)>, // (student_name,student_pubkey total_score , number of deserved tokens)
    pub class4_info: Vec<(String, Pubkey, i64, u8)>, // (student_name,student_pubkey total_score , number of deserved tokens)
}
pub const STUDENT_NAME_MAX: usize = 24;
pub const STUDENTS_PER_CLASS: usize = 20;
pub const STUDENT_ENTRY_SIZE: usize = 4 + STUDENT_NAME_MAX + 32 + 8 + 1;
pub const CLASS_INFO_SIZE: usize = 4 + (STUDENTS_PER_CLASS * STUDENT_ENTRY_SIZE);
pub const ALL_CLASSES_SIZE: usize = 4 * CLASS_INFO_SIZE;

impl Students {
    pub const LEN: usize = 8 + (4 * (4 + (STUDENTS_PER_CLASS * STUDENT_ENTRY_SIZE)));
}
