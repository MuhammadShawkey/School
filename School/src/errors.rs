use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Student name is too long. Maximum 32 characters allowed.")]
    NameTooLong,

    #[msg("Name contains non-alphabetic characters. Only A-Z and a-z are allowed.")]
    InvalidCharacters,

    #[msg("Invalid class index. Must be between 0 and 3.")]
    InvalidClassIndex,

    #[msg("You Are Not Authorized to call this function")]
    UnAuthorized,

    #[msg("Student already exists in the class.")]
    AlreadyAssigned,

    #[msg("Class has reached its maximum capacity.")]
    ClassFull,

    #[msg("Student not found in the class.")]
    StudentNotFound,

    #[msg("Student with this name already exists.")]
    StudentAlreadyExists,

    #[msg("Reason is too long , please shorten it")]
    ReasonTooLong,

    #[msg("Student does not deserve a token.")]
    StudentDoesNotdeservedatoken,
    #[msg("Token isn't owned by this account.")]
    InvalidTokenAccountOwner,
}
