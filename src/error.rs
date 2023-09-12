use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Uninitialized")]
    Uninitialized {},
    #[error("Cw721 Already Linked")]
    Cw721AlreadyLinked {},

    #[error("Invalid token reply ID")]
    InvalidTokenReplyId {},
}