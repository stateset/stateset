use cosmwasm_std::{
    entry_point, to_binary, BankMsg, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};

use crate::error::ContractError;
use crate::msg::{ConfigResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{config, config_read, State};

// Instantiate the Smart Contract for a Stateset Group 
#[entry_point]
pub fn instantiate(
    deps: DepsMut, // Mutable Deps
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg, // InstantiateMsg from msg.rs crate
) -> Result<Response, ContractError> {

    // State from state.msg crate
    let state = State {
        provider: info.sender.clone(), // creator of the proof is the provider
        did: info.did, // funds put up as collateral 
        payload: msg.payload, // payload for the proof
        status: msg.status // status for the proof
    };

    // Save the State defined in state.rs crate to the state
    config(deps.storage).save(&state)?;

    Ok(Response::default())
}


// Entry Point for Execution of the Proof
#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg, // ExecuteMsg as defined in the msg.rs crate
) -> Result<Response, ContractError> {
    match msg {
        // ExecuteMsg::Transfer from msg.rs crate
        ExecuteMsg::Verify { proof } => execute_verify(deps, env, info, recipient),
    }
}


// Transfer the Option
pub fn execute_verify(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    payload: String, // payload of the proof
) -> Result<Response, ContractError> {

    // Check Contract Error Conditions
    // ensure msg sender is the owner
    // load the state from deps.storage imported from cosmwasm_std crate
    let mut state = config(deps.storage).load()?;
    if info.sender != state.owner {
        // ContractError::Unauthorized from error.rs crate
        return Err(ContractError::Unauthorized {});
    }

    // set new owner on state as defined in the state.rs crate
    state.proof = deps.api.proof_validate(&proof)?;
    // save the state to storage
    config(deps.storage).save(&state)?;

    let mut res = Response::new();
    let status = 'valid';
    res.add_attribute("action", "verify");
    res.add_attribute("status", status);
    Ok(res)
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::ListProofs {} => to_binary(&query_list(deps)?),
        QueryMsg::Proof { id } => to_binary(&query_proof(deps, id)?),
    }
}


fn query_proof(deps: Deps) -> StdResult<ProofResponse> {
    let query = ProofQuery::ProofId {}.into();
    let ProofIdResponse { proof_id } = deps.querier.query(&query)?;
    Ok(ProofResponse { proof_id })
}

fn query_list(deps: Deps) -> StdResult<ListProofsResponse> {
    let proofs: StdResult<Vec<_>> = PROOF_INFO
        .range(deps.storage, None, None, Order::Ascending)
        .map(|r| r.map(|(_, v)| v))
        .collect();
    Ok(ListProofsResponse {
        proofs: proofs?,
    })
}