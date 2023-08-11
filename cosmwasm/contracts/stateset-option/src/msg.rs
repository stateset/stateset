use crate::state::State;
use cosmwasm_std::Coin;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};


// Define Messages in the msg.rs crate
// Define InstantiateMsg
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    // owner and creator come from env
    // collateral comes from env
    pub counter_offer: Vec<Coin>,
    pub expires: u64,
}

//Define ExecuteMsg

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    // ExecuteMsg::Transfer
    /// Owner can transfer to a new owner
    Transfer { recipient: String },
    // ExecuteMsg::Execute
    /// Owner can post counter_offer on unexpired option to execute and get the collateral
    Execute {},
    // ExecuteMsg::Burn
    /// Burn will release collateral if expired
    Burn {},
}

//Define Query Msg
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Config {},
}

// We define a custom struct for each query response
pub type ConfigResponse = State;
