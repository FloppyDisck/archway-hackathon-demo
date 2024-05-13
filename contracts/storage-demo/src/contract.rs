#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, QueryRequest, Response, StdError,
    StdResult, WasmQuery,
};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, Foo, CONFIG, ITEMS, NUMBERS, REGISTRY};

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
    let config = Config::new(info.sender, msg.data_size);
    CONFIG.save(deps.storage, &config)?;
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
            let mut items = vec![];
            for i in 0..count {
                items.push(Foo::new(i));
            }
            ITEMS.save(deps.storage, &items)?;
        }
        ExecuteMsg::AddItem { item } => {
            ITEMS.update(deps.storage, |mut items| {
                items.push(Foo::new(item));
                Ok::<_, StdError>(items)
            })?;
        }
        ExecuteMsg::ReadItem { name } => {
            let items = ITEMS.load(deps.storage)?;
            items.iter().find(|foo| foo.name == name).unwrap();
        }
        ExecuteMsg::StoreNumber { iter, numb } => {
            for i in 0..iter {
                NUMBERS.save(deps.storage, i, &numb)?;
            }
        }
        ExecuteMsg::ReadNumber { iter } => {
            for i in 0..iter {
                NUMBERS.load(deps.storage, i)?;
            }
        }
        ExecuteMsg::SetAdmin { admin } => {
            CONFIG.update(deps.storage, |mut config| {
                config.admin = admin;
                Ok::<_, StdError>(config)
            })?;
        }
        ExecuteMsg::GetAdmin {} => {
            CONFIG.load(deps.storage)?.admin;
        }
        ExecuteMsg::SetArchIdRegistry { contract } => {
            REGISTRY.save(deps.storage, &contract)?;
        }
        ExecuteMsg::TestArchID { addr } => {
            let _response: archid_registry::msg::ResolveAddressResponse = deps
                .querier
                .query(&QueryRequest::Wasm(WasmQuery::Smart {
                    contract_addr: REGISTRY.load(deps.storage)?,
                    msg: to_json_binary(&archid_registry::msg::QueryMsg::ResolveAddress {
                        address: addr,
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
