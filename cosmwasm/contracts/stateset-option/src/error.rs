use cosmwasm_std::{Coin, StdError};
use thiserror::Error;


// Define Error Messages in the error.rs crate
// Contract Error
#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    // ContractError::OptionExpired
    #[error("expired option (expired {expired:?})")]
    OptionExpired { expired: u64 },

    // ContractError::OptionNotExpired
    #[error("not expired option (expires {expires:?})")]
    OptionNotExpired { expires: u64 },

    // ContractError::Unauthorized
    #[error("unauthorized")]
    Unauthorized {},

    // ContractError::CounterOfferMismatch
    #[error("must send exact counter offer (offer {offer:?}, counter_offer: {counter_offer:?})")]
    CounterOfferMismatch {
        offer: Vec<Coin>,
        counter_offer: Vec<Coin>,
    },

    // ContractError::FundsSendWithBurn
    #[error("do not send funds with burn")]
    FundsSentWithBurn {},
}
