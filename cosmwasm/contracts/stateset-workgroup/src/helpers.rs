use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

use cosmwasm_std::{to_binary, Addr, CosmosMsg, StdResult, WasmMsg};
use workgroup::{WorkgroupContract, Member};

use crate::msg::ExecuteMsg;

/// StatesetWorkGroupContract is a wrapper around WorkgroupContract that provides a lot of helpers
/// for working with stateset-workgroup contracts.
///
/// It extends StatesetWorkgroupContract to add the extra calls from stateset-workgroup.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct StatesetWorkgroupGroupContract(pub WorkgroupContract);

impl Deref for StatesetWorkgroupContract {
    type Target = WorkgroupContract;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl StatesetWorkgroupContract {
    pub fn new(addr: Addr) -> Self {
        StatesetWorkgroupContract(WorkgroupContract(addr))
    }

    fn encode_msg(&self, msg: ExecuteMsg) -> StdResult<CosmosMsg> {
        Ok(WasmMsg::Execute {
            contract_addr: self.addr().into(),
            msg: to_binary(&msg)?,
            funds: vec![],
        }
        .into())
    }

    pub fn update_members(&self, remove: Vec<String>, add: Vec<Member>) -> StdResult<CosmosMsg> {
        let msg = ExecuteMsg::UpdateMembers { remove, add };
        self.encode_msg(msg)
    }
}
