use cosmwasm_std::{
    entry_point, to_binary, BankMsg, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};

use crate::error::ContractError;
use crate::msg::{ConfigResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{config, config_read, State};

// Instantiate the Smart Contract for a Simple Option
#[entry_point]
pub fn instantiate(
    deps: DepsMut, // Mutable Deps
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg, // InstantiateMsg from msg.rs crate
) -> Result<Response, ContractError> {

    
    if msg.expires <= env.block.height {

        // ContractError::OptionExpired as define from the error.rs crate
        return Err(ContractError::OptionExpired {
            expired: msg.expires,
        });
    }

    // State from state.msg crate
    let state = State {
        creator: info.sender.clone(), // creator of the option is the sender
        owner: info.sender.clone(), // owner of the option is the sender
        collateral: info.funds, // funds put up as collateral 
        counter_offer: msg.counter_offer, // conunter_offer for the option
        expires: msg.expires, // expiration for the option
    };

    // Save the State defined in state.rs crate to the state
    config(deps.storage).save(&state)?;

    Ok(Response::default())
}


// Entry Point for Execution of the Option
#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg, // ExecuteMsg as defined in the msg.rs crate
) -> Result<Response, ContractError> {
    match msg {
        // ExecuteMsg::Transfer from msg.rs crate
        ExecuteMsg::Transfer { recipient } => execute_transfer(deps, env, info, recipient),
        // ExecuteMsg::Execute from msg.rs crate
        ExecuteMsg::Execute {} => execute_execute(deps, env, info),
        // ExecuteMsg::Burn from msg.rs crate
        ExecuteMsg::Burn {} => execute_burn(deps, env, info),
    }
}

// Transfer the Option
pub fn execute_transfer(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    recipient: String, // Recipient of the Option
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
    state.owner = deps.api.addr_validate(&recipient)?;
    // save the state to storage
    config(deps.storage).save(&state)?;

    let mut res = Response::new();
    res.add_attribute("action", "transfer");
    res.add_attribute("owner", recipient);
    Ok(res)
}

// execute function
pub fn execute_execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {

    // Check the Contract Conditions Before Executing the State Change

    // ensure msg sender is the owner
    // load the current state to be use to check the conditions
    let state = config(deps.storage).load()?;
    if info.sender != state.owner {
        // ContractError::Unauthorized from error.rs crate
        return Err(ContractError::Unauthorized {});
    }

    // ensure not expired
    // check to see if the curreny block height is greater than the expiration block height
    if env.block.height >= state.expires {
        // ContractError::OptionExpired from error.rs crate
        return Err(ContractError::OptionExpired {
            // return the expired block height
            expired: state.expires,
        });
    }

    // ensure sending proper counter_offer
    if info.funds != state.counter_offer {
        // ContractError::CounterOfferMismatched from error.rs crate
        return Err(ContractError::CounterOfferMismatch {
            // return the offer from the sender against the counter_offer value
            offer: info.funds,
            counter_offer: state.counter_offer,
        });
    }

    // Generate a Response from the Network
    // release counter_offer to creator
    let mut res = Response::new();
    res.add_message(BankMsg::Send {
        to_address: state.creator.to_string(),
        amount: state.counter_offer,
    });

    // release collateral to sender
    res.add_message(BankMsg::Send {
        to_address: state.owner.to_string(),
        amount: state.collateral,
    });

    // delete the option
    config(deps.storage).remove();

    res.add_attribute("action", "execute");
    Ok(res)
}

// Burn the Option
pub fn execute_burn(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    // ensure is expired
    // load the state
    let state = config(deps.storage).load()?;


    if env.block.height < state.expires {
        // ContractError::OprionNotExpired error as defined in error.rs crate
        return Err(ContractError::OptionNotExpired {
            expires: state.expires,
        });
    }

    // ensure sending proper counter_offer
    if !info.funds.is_empty() {
        // ContractError::FundsSentWithBurn error as defined in error.rs crate
        return Err(ContractError::FundsSentWithBurn {});
    }

    // release collateral to creator
    let mut res = Response::new();
    res.add_message(BankMsg::Send {
        to_address: state.creator.to_string(),
        amount: state.collateral,
    });

    // delete the option
    config(deps.storage).remove();

    res.add_attribute("action", "burn");
    Ok(res)
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
    }
}

fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let state = config_read(deps.storage).load()?;
    Ok(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{attr, coins, CosmosMsg};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies(&[]);

        let msg = InstantiateMsg {
            counter_offer: coins(40, "ETH"),
            expires: 100_000,
        };
        let info = mock_info("creator", &coins(1, "BTC"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query_config(deps.as_ref()).unwrap();
        assert_eq!(100_000, res.expires);
        assert_eq!("creator", res.owner.as_str());
        assert_eq!("creator", res.creator.as_str());
        assert_eq!(coins(1, "BTC"), res.collateral);
        assert_eq!(coins(40, "ETH"), res.counter_offer);
    }

    #[test]
    fn transfer() {
        let mut deps = mock_dependencies(&[]);

        let msg = InstantiateMsg {
            counter_offer: coins(40, "ETH"),
            expires: 100_000,
        };
        let info = mock_info("creator", &coins(1, "BTC"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // random cannot transfer
        let info = mock_info("anyone", &[]);
        let err =
            execute_transfer(deps.as_mut(), mock_env(), info, "anyone".to_string()).unwrap_err();
        match err {
            ContractError::Unauthorized {} => {}
            e => panic!("unexpected error: {}", e),
        }

        // owner can transfer
        let info = mock_info("creator", &[]);
        let res = execute_transfer(deps.as_mut(), mock_env(), info, "someone".to_string()).unwrap();
        assert_eq!(res.attributes.len(), 2);
        assert_eq!(res.attributes[0], attr("action", "transfer"));

        // check updated properly
        let res = query_config(deps.as_ref()).unwrap();
        assert_eq!("someone", res.owner.as_str());
        assert_eq!("creator", res.creator.as_str());
    }

    #[test]
    fn execute() {
        let mut deps = mock_dependencies(&[]);

        let amount = coins(40, "ETH");
        let collateral = coins(1, "BTC");
        let expires = 100_000;
        let msg = InstantiateMsg {
            counter_offer: amount.clone(), // clone the amount for the counter_offer
            expires: expires,
        };
        let info = mock_info("creator", &collateral);

        // we can just call .unwrap() to assert this was a success
        let _ = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // set new owner
        let info = mock_info("creator", &[]);
        let _ = execute_transfer(deps.as_mut(), mock_env(), info, "owner".to_string()).unwrap();

        // random cannot execute
        let info = mock_info("creator", &amount);
        let err = execute_execute(deps.as_mut(), mock_env(), info).unwrap_err();
        match err {
            ContractError::Unauthorized {} => {}
            e => panic!("unexpected error: {}", e),
        }

        // expired cannot execute
        let info = mock_info("owner", &amount);
        let mut env = mock_env();
        env.block.height = 200_000; // Block 200_000 is > 100_000 as defined in expires
        let err = execute_execute(deps.as_mut(), env, info).unwrap_err();
        match err {
            ContractError::OptionExpired { expired } => assert_eq!(expired, expires),
            e => panic!("unexpected error: {}", e),
        }

        // bad counter_offer cannot execute
        let msg_offer = coins(39, "ETH"); // 39 ETH is not = 40 ETH
        let info = mock_info("owner", &msg_offer);
        let err = execute_execute(deps.as_mut(), mock_env(), info).unwrap_err();
        match err {
            ContractError::CounterOfferMismatch {
                offer,
                counter_offer,
            } => {
                assert_eq!(msg_offer, offer);
                assert_eq!(amount, counter_offer);
            }
            e => panic!("unexpected error: {}", e),
        }

        // proper execution
        let info = mock_info("owner", &amount);
        let res = execute_execute(deps.as_mut(), mock_env(), info).unwrap();
        assert_eq!(res.messages.len(), 2);
        assert_eq!(
            res.messages[0],
            CosmosMsg::Bank(BankMsg::Send {
                to_address: "creator".into(),
                amount,
            })
        );
        assert_eq!(
            res.messages[1],
            CosmosMsg::Bank(BankMsg::Send {
                to_address: "owner".into(),
                amount: collateral,
            })
        );

        // check deleted
        let _ = query_config(deps.as_ref()).unwrap_err();
    }

    #[test]
    fn burn() {
        let mut deps = mock_dependencies(&[]);

        let counter_offer = coins(40, "ETH");
        let collateral = coins(1, "BTC");
        let msg_expires = 100_000;
        let msg = InstantiateMsg {
            counter_offer: counter_offer.clone(),
            expires: msg_expires,
        };
        let info = mock_info("creator", &collateral);

        // we can just call .unwrap() to assert this was a success
        let _ = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // set new owner
        let info = mock_info("creator", &[]);
        let _ = execute_transfer(deps.as_mut(), mock_env(), info, "owner".to_string()).unwrap();

        // non-expired cannot execute
        let info = mock_info("anyone", &[]);
        let err = execute_burn(deps.as_mut(), mock_env(), info).unwrap_err();
        match err {
            ContractError::OptionNotExpired { expires } => assert_eq!(expires, msg_expires),
            e => panic!("unexpected error: {}", e),
        }

        // with funds cannot execute
        let info = mock_info("anyone", &counter_offer);
        let mut env = mock_env();
        env.block.height = 200_000;
        let err = execute_burn(deps.as_mut(), env, info).unwrap_err();
        match err {
            ContractError::FundsSentWithBurn {} => {}
            e => panic!("unexpected error: {}", e),
        }

        // expired returns funds
        let info = mock_info("anyone", &[]);
        let mut env = mock_env();
        env.block.height = 200_000;
        let res = execute_burn(deps.as_mut(), env, info).unwrap();
        assert_eq!(res.messages.len(), 1);
        assert_eq!(
            res.messages[0],
            CosmosMsg::Bank(BankMsg::Send {
                to_address: "creator".into(),
                amount: collateral,
            })
        );

        // check deleted
        let _ = query_config(deps.as_ref()).unwrap_err();
    }
}
