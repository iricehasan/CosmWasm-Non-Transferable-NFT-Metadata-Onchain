#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

use cosmwasm_std::{
    to_binary, wasm_instantiate, Empty, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult, SubMsg, Addr
};

use crate::msg::{InstantiateMsg, ExecuteMsg, QueryMsg};
use crate::state::{Config, CONFIG};
use crate::error::ContractError;

use cw721_base::{
    ExecuteMsg as Cw721ExecuteMsg, InstantiateMsg as Cw721InstantiateMsg, Extension, MintMsg
};
use cw_utils::parse_reply_instantiate_data;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {

    let config = Config {
        cw721_address: None,
        token_uri: msg.token_uri.clone(),
        extension: msg.extension.clone(),
        token_id: 0,
    };

    CONFIG.save(deps.storage, &config)?;

 
    let cw721_init_msg = Cw721InstantiateMsg {
        name: "Crowdfunding Participation".to_owned(),
        symbol: "CP".to_owned(),
        minter: env.contract.address.to_string(),
    };     

    let sub_msg = SubMsg::reply_on_success(
        wasm_instantiate(
            msg.cw721_code_id, // stored code id will be provided via Instantiate Msg
            &cw721_init_msg,
            vec![],
            "quadratic funding nft contract".to_owned(),
        )
        .unwrap(),
        1,
    );

    Ok(Response::new().add_submessage(sub_msg))
}

// this reply callback is triggered from cw721 instantiation submessage
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    let mut config: Config = CONFIG.load(deps.storage)?;

    if config.cw721_address != None {
        return Err(ContractError::Cw721AlreadyLinked {});
    }

    if msg.id != 1 {
        return Err(ContractError::InvalidTokenReplyId {});
    }

    let reply = parse_reply_instantiate_data(msg).unwrap();
    config.cw721_address = Addr::unchecked(reply.contract_address).into();
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Mint {} => mint(deps, env, info),
    }
}

pub fn mint(deps: DepsMut, _env: Env, info: MessageInfo) -> Result<Response, ContractError> {

    let config = CONFIG.load(deps.storage)?;

    if config.cw721_address == None {
        return Err(ContractError::Uninitialized {});
    }

    // learn how to use whitelist here

    let mint_msg = Cw721ExecuteMsg::<Extension, Empty>::Mint(MintMsg::<Extension> {
        token_id: config.token_id.to_string(),
        owner: info.sender.clone().to_string(),
        token_uri: config.token_uri.clone().into(),
        extension: config.extension.clone(),
    });

    // learn how to use mint_msg then increment token_id by 1

    Ok(Response::new()
        .add_attribute("action", "mint"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
    }
}

/// Returns contract configuration
fn query_config(deps: Deps) -> StdResult<Config> {
    let config = CONFIG.load(deps.storage)?;
    Ok(config)
}