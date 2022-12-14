use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Insufficient funds sent")]
    InsufficientFundsSent {},

    #[error("Invalid coin denomination, expected {expected} got {actual}")]
    InvalidDenom { expected: String, actual: String },

    #[error("Name does not exist (name {name})")]
    NameNotExists { name: String },

    #[error("No bids for name")]
    NameNoBids,

    #[error("User Already Registered")]
    UserAlreadyRegistered{ name: String },

    #[error("Name has been taken (name {name})")]
    NameTaken { name: String },

    #[error("Name too short (length {length} min_length {min_length})")]
    NameTooShort { length: u64, min_length: u64 },

    #[error("Name too long (length {length} min_length {max_length})")]
    NameTooLong { length: u64, max_length: u64 },

    #[error("Name needs suffix {suffix}")]
    NameNeedsSuffix { suffix: String },

    #[error("Invalid character (char {c})")]
    InvalidCharacter { c: char },

    #[error("Years must be positive integer")]
    YearsMustBePositive {},

    #[error("Not implemented yet")]
    NotImplemented {},
}