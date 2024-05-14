#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, QueryRequest, Response,
    StdResult, WasmQuery,
};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, Foo, CONFIG, ITEMS, NUMBERS, TOKEN, ADMIN};

/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:storage-demo";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let config = Config::new(msg.data_size);
    CONFIG.save(deps.storage, &config)?;
    ADMIN.save(deps.storage, &info.sender)?;
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::StoreItems { count } => {
            for i in 0..count {
                ITEMS.save(deps.storage, i, &Foo::new(i))?;
            }
        }
        ExecuteMsg::AddItem { item } => {
            ITEMS.save(deps.storage, item, &Foo::new(item))?;
        }
        ExecuteMsg::ReadItem { name } => {
            ITEMS.load(deps.storage, name.parse().unwrap())?;
        }


        ExecuteMsg::StoreNumber { iter, numb } => {
            let n = numb.u128();
            for i in 0..iter {
                NUMBERS.save(deps.storage, i, &n)?;
            }
        }
        ExecuteMsg::ReadNumber { iter } => {
            for i in 0..iter {
                NUMBERS.load(deps.storage, i)?;
            }
        }


        ExecuteMsg::SetAdmin { admin } => {
            ADMIN.save(deps.storage, &admin)?;
        }
        ExecuteMsg::GetAdmin {} => {
            ADMIN.load(deps.storage)?;
        }


        ExecuteMsg::SetArchIdToken { contract } => {
            TOKEN.save(deps.storage, &contract)?;
        }
        ExecuteMsg::TestArchID { addr } => {
            let _response: cw721::TokensResponse = deps
                .querier
                .query(&QueryRequest::Wasm(WasmQuery::Smart {
                    contract_addr: TOKEN.load(deps.storage)?,
                    msg: to_json_binary(&cw721::Cw721QueryMsg::Tokens {
                        owner: addr.to_string(),
                        start_after: None,
                        limit: None,
                    })
                    .unwrap(),
                }))
                .unwrap();
        }
    }

    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    Ok(Binary::default())
}

#[cfg(test)]
mod tests {}
