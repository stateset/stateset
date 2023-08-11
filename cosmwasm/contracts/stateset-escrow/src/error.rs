use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    // unauthorized 
    #[error("Unauthorized")]
    Unauthorized {},

    // tokens whitelisted can only be used in the escrow contract
    #[error("Only accepts tokens in the cw20_whitelist")]
    NotInWhitelist {},

    // escrow contract expired
    #[error("Escrow is expired")]
    Expired {},

    // balance of escrow contract is empty
    #[error("Send some coins to create an escrow")]
    EmptyBalance {},

    // escrow id in use
    #[error("Escrow id already in use")]
    AlreadyInUse {},

    // need to set escrow recipient
    #[error("Recipient is not set")]
    RecipientNotSet {},
}