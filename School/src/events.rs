use anchor_lang::prelude::*;

#[event]
pub struct StudentEvaluated {
    pub student_name: String,
    pub class_index: u8,
    pub score: i64,
    pub reason: String,
    pub total_score: i64,
}

#[event]
pub struct StudentRewarded {
    pub student_name: String,
    pub reward: u64,
}

#[event]
pub struct StudentTerminated {
    pub student_name: String,
    pub class_index: u8,
    pub total_score: i64,
}
