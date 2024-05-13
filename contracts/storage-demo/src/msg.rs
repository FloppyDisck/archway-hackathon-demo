use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128};

#[cw_serde]
pub struct InstantiateMsg {
    pub data_size: u64,
}

#[cw_serde]
pub enum ExecuteMsg {
    // **** Map vs Item for storing item lists ****
    /// TX: Store n items
    StoreItems {
        count: u64,
    },
    /// TX: Store one item
    AddItem {
        item: u64,
    },
    /// Query: Read one of the stored items
    ReadItem {
        name: String,
    },

    // **** Uint128 ****
    /// TX: Store a commonly used item in dApp development
    StoreNumber {
        iter: u64,
        numb: Uint128,
    },
    /// Query: Read stored value
    ReadNumber {
        iter: u64,
    },

    // **** Config Storage ****
    /// Store the config
    /// TX: Sets the config admin
    SetAdmin {
        admin: Addr,
    },
    /// Query: Get the config admin
    GetAdmin {},

    // **** NetWars reference ****
    SetArchIdRegistry {
        contract: String,
    },
    /// Test archid query
    TestArchID {
        addr: Addr,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}
