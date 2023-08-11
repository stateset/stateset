use crate::state::State;
use cosmwasm_std::Coin;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};


// Define Messages in the msg.rs crate
// Define InstantiateMsg
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    // provider comes from env
    pub did: String,
    pub payload: String,
    pub status: String,
}


//Define ExecuteMsg

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {

    // ExecuteMsg::VerifyW
    /// Requester can verify proof
    Verify { proof: String }

}

//Define Query Msg
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Config {},
}

// We define a custom struct for each query response
pub type ConfigResponse = State;
