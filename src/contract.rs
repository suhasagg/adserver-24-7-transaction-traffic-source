use cosmwasm_std::{
    attr, to_binary, Binary, Deps, DepsMut, Env, Event, MessageInfo, Response, StdError, StdResult,
};
use crate::msg::{
    Ad, ExecuteMsg, InitMsg, QueryAdResponse, QueryAllAdsResponse, QueryMsg, TotalViewsResponse,
};
use crate::state::{load_state, save_state, State};

/// Initializes the contract state.
/// Sets up an empty list of ads and zero total views.
#[cfg_attr(not(feature = "library"), cosmwasm_std::entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InitMsg,
) -> StdResult<Response> {
    let state = State {
        ads: vec![],
        total_views: 0,
        plt_address: "".to_string(),
    };
    save_state(deps.storage, &state)?;
    Ok(Response::new().add_attribute("method", "instantiate"))
}

/// The main entry point for execute calls.
/// Routes the incoming message to the appropriate function.
#[cfg_attr(not(feature = "library"), cosmwasm_std::entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::AddAd {
            id,
            image_url,
            target_url,
            reward_address,
        } => add_ad(deps, env, info, id, image_url, target_url, reward_address),
        ExecuteMsg::ServeAd { id } => serve_ad(deps, env, id),
        ExecuteMsg::DeleteAd { id } => delete_ad(deps, id),
        ExecuteMsg::BatchServeAds { ids } => batch_serve_ads(deps, env, ids),
    }
}

/// Adds a new ad to the list, ensuring no duplicate ID is added.
fn add_ad(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    id: String,
    image_url: String,
    target_url: String,
    reward_address: String,
) -> StdResult<Response> {
    let mut state = load_state(deps.storage)?;

    // Check if an ad with the same ID already exists
    if state.ads.iter().any(|ad| ad.id == id) {
        return Err(StdError::generic_err("Ad with this ID already exists"));
    }

    let ad = Ad {
        id: id.clone(),
        image_url: image_url.clone(),
        target_url: target_url.clone(),
        views: 0,
        reward_address: reward_address.clone(),
    };

    state.ads.push(ad);
    save_state(deps.storage, &state)?;

    let attributes = vec![
        attr("action", "add_ad"),
        attr("ad_id", id),
        attr("reward_address", reward_address),
        attr("image_url", image_url),
        attr("target_url", target_url),
    ];

    let event = Event::new("add_ad").add_attributes(attributes);
    Ok(Response::new().add_event(event))
}

/// Increments the view count of a specific ad by its ID.
fn serve_ad(deps: DepsMut, _env: Env, id: String) -> StdResult<Response> {
    let mut state = load_state(deps.storage)?;
    
    // Perform all modifications and extract needed values in a separate scope
    let (views, image_url, target_url) = {
        let ad = state
            .ads
            .iter_mut()
            .find(|ad| ad.id == id)
            .ok_or_else(|| StdError::generic_err("Cannot serve ad: ID not found"))?;
        
        ad.views += 1;
        state.total_views += 1;
      
        (ad.views, ad.image_url.clone(), ad.target_url.clone())
    }; 
    
    save_state(deps.storage, &state)?;

    let attributes = vec![
        attr("action", "serve_ad"),
        attr("ad_id", id),
        attr("views", views.to_string()),
        attr("image_url", image_url),
        attr("target_url", target_url),
    ];

    let event = Event::new("serve_ad").add_attributes(attributes);
    Ok(Response::new().add_event(event))
}


/// Deletes an ad from the list by its ID.
fn delete_ad(deps: DepsMut, id: String) -> StdResult<Response> {
    let mut state = load_state(deps.storage)?;
    let ad_index = state.ads.iter().position(|ad| ad.id == id);

    if let Some(index) = ad_index {
        state.ads.remove(index);
    } else {
        return Err(StdError::generic_err("Cannot delete: Ad not found"));
    }

    save_state(deps.storage, &state)?;

    let event = Event::new("delete_ad")
        .add_attribute("action", "delete_ad")
        .add_attribute("ad_id", id);
    Ok(Response::new().add_event(event))
}

/// Serves multiple ads at once, incrementing their view counts if they exist.
fn batch_serve_ads(deps: DepsMut, _env: Env, ids: Vec<String>) -> StdResult<Response> {
    let mut state = load_state(deps.storage)?;
    let mut events = Vec::new();

    for ad_id in ids {
        if let Some(ad) = state.ads.iter_mut().find(|ad| ad.id == ad_id) {
            ad.views += 1;
            state.total_views += 1;
            let event = Event::new("serve_ad")
                .add_attribute("action", "serve_ad")
                .add_attribute("ad_id", ad.id.clone())
                .add_attribute("views", ad.views.to_string())
                .add_attribute("image_url", ad.image_url.clone())
                .add_attribute("target_url", ad.target_url.clone());
            events.push(event);
        }
    }

    save_state(deps.storage, &state)?;
    Ok(Response::new().add_events(events))
}

/// Query entry point, routes queries to the appropriate handler.
#[cfg_attr(not(feature = "library"), cosmwasm_std::entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Ad { id } => query_ad(deps, id),
        QueryMsg::Ads => query_all_ads(deps),
        QueryMsg::TotalViews => query_total_views(deps),
    }
}

/// Queries a single ad by its ID and returns its details.
fn query_ad(deps: Deps, id: String) -> StdResult<Binary> {
    let state = load_state(deps.storage)?;
    if let Some(ad) = state.ads.iter().find(|&ad| ad.id == id) {
        to_binary(&QueryAdResponse {
            id: ad.id.clone(),
            image_url: ad.image_url.clone(),
            target_url: ad.target_url.clone(),
            views: ad.views,
            reward_address: ad.reward_address.clone(),
        })
    } else {
        Err(StdError::generic_err("Ad not found"))
    }
}

/// Returns all ads currently stored.
fn query_all_ads(deps: Deps) -> StdResult<Binary> {
    let state = load_state(deps.storage)?;
    let ads: Vec<QueryAdResponse> = state
        .ads
        .iter()
        .map(|ad| QueryAdResponse {
            id: ad.id.clone(),
            image_url: ad.image_url.clone(),
            target_url: ad.target_url.clone(),
            views: ad.views,
            reward_address: ad.reward_address.clone(),
        })
        .collect();

    to_binary(&QueryAllAdsResponse { ads })
}

/// Returns the total number of views across all ads.
fn query_total_views(deps: Deps) -> StdResult<Binary> {
    let state = load_state(deps.storage)?;
    to_binary(&TotalViewsResponse {
        total_views: state.total_views,
    })
}
