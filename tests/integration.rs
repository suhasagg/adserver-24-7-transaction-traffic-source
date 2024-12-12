use cosmwasm_std::Addr;
use cw_multi_test::{App, Contract, ContractWrapper, Executor};
use adserver::contract::{execute, instantiate, query};
use adserver::msg::{ExecuteMsg, InitMsg, QueryAdResponse, QueryAllAdsResponse, QueryMsg};

fn mock_app() -> App {
    App::default()
}

fn contract_adserver() -> Box<dyn Contract<cosmwasm_std::Empty>> {
    let contract = ContractWrapper::new(execute, instantiate, query);
    Box::new(contract)
}

#[test]
fn test_integration_flow() {
    let mut app = mock_app();
    let code_id = app.store_code(contract_adserver());

    // Instantiate the contract
    let contract_addr = app
        .instantiate_contract(
            code_id,
            Addr::unchecked("creator"),
            &InitMsg {},
            &[],
            "AdServer",
            None,
        )
        .unwrap();

    // Add an ad
    let ad_id = "ad_integration_1".to_string();
    let image_url = "https://example.com/int_image".to_string();
    let target_url = "https://example.com/int_target".to_string();
    let reward_address = "int_reward".to_string();

    // Add the ad
    app.execute_contract(
        Addr::unchecked("creator"),
        contract_addr.clone(),
        &ExecuteMsg::AddAd {
            id: ad_id.clone(),
            image_url: image_url.clone(),
            target_url: target_url.clone(),
            reward_address: reward_address.clone(),
        },
        &[],
    )
    .unwrap();

    // Serve the ad
    app.execute_contract(
        Addr::unchecked("creator"),
        contract_addr.clone(),
        &ExecuteMsg::ServeAd { id: ad_id.clone() },
        &[],
    )
    .unwrap();

    // Query the ad
    let ad: QueryAdResponse = app
        .wrap()
        .query_wasm_smart(
            contract_addr.clone(),
            &QueryMsg::Ad { id: ad_id.clone() },
        )
        .unwrap();
    assert_eq!(ad.views, 1);

    // Batch serve multiple ads (including the existing one)
    let ad_id2 = "ad_integration_2".to_string();
    app.execute_contract(
        Addr::unchecked("creator"),
        contract_addr.clone(),
        &ExecuteMsg::AddAd {
            id: ad_id2.clone(),
            image_url: "https://example.com/int_image2".into(),
            target_url: "https://example.com/int_target2".into(),
            reward_address: "int_reward2".into(),
        },
        &[],
    )
    .unwrap();

    app.execute_contract(
        Addr::unchecked("creator"),
        contract_addr.clone(),
        &ExecuteMsg::BatchServeAds {
            ids: vec![ad_id.clone(), ad_id2.clone()],
        },
        &[],
    )
    .unwrap();

    // Query the ads to check if both served again
    let ad1: QueryAdResponse = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &QueryMsg::Ad { id: ad_id.clone() })
        .unwrap();
    let ad2: QueryAdResponse = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &QueryMsg::Ad { id: ad_id2.clone() })
        .unwrap();

    assert_eq!(ad1.views, 2); // served once before + once in batch
    assert_eq!(ad2.views, 1); // served once in batch

    // Delete an ad
    app.execute_contract(
        Addr::unchecked("creator"),
        contract_addr.clone(),
        &ExecuteMsg::DeleteAd { id: ad_id.clone() },
        &[],
    )
    .unwrap();

    // Query all ads to ensure the deleted ad is removed
    let ads: QueryAllAdsResponse = app
        .wrap()
        .query_wasm_smart(contract_addr, &QueryMsg::Ads)
        .unwrap();
    assert_eq!(ads.ads.len(), 1);
    assert_eq!(ads.ads[0].id, ad_id2);
}

