use cosmwasm_std::{Coin, StdError};
use thiserror::Error;


// Define Error Messages in the error.rs crate
// Contract Error
#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    // ContractError::ProofExpired
    #[error("expired proof (expired {expired:?})")]
    ProofExpired { expired: u64 },


    // ContractError::ProofExpired
    #[error("proof verification failed (expired {expired:?})")]
    VerificationFailed {},

    // ContractError::Unauthorized
    #[error("unauthorized")]
    Unauthorized {},
}
