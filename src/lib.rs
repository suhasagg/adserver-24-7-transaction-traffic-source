#[cfg(feature = "library")]
use cosmwasm_std::entry_point;

pub mod contract;
pub mod msg;
pub mod state;

#[cfg(test)]
mod tests {
    use cosmwasm_std::Addr;
    use cw_multi_test::{App, Contract, ContractWrapper, Executor};

    use crate::contract::{execute, instantiate, query};
    use crate::msg::{ExecuteMsg, InitMsg, QueryAdResponse, QueryAllAdsResponse, QueryMsg};

    fn mock_app() -> App {
        App::default()
    }

    fn contract_adserver() -> Box<dyn Contract<cosmwasm_std::Empty>> {
        let contract = ContractWrapper::new(execute, instantiate, query);
        Box::new(contract)
    }

    #[test]
    fn test_adserver_basic_operations() {
        let mut app = mock_app();
        let code_id = app.store_code(contract_adserver());

        let contract_addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked("owner"),
                &InitMsg {},
                &[],
                "AdServer",
                None,
            )
            .unwrap();

        // Add an ad
        let ad_id = "ad1".to_string();
        let image_url = "https://example.com/image1".to_string();
        let target_url = "https://example.com/landing".to_string();
        let reward_address = "reward1".to_string();

        app.execute_contract(
            Addr::unchecked("owner"),
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
            Addr::unchecked("owner"),
            contract_addr.clone(),
            &ExecuteMsg::ServeAd { id: ad_id.clone() },
            &[],
        )
        .unwrap();

        // Query the ad to check if it has been served
        let ad: QueryAdResponse = app
            .wrap()
            .query_wasm_smart(
                contract_addr.clone(),
                &QueryMsg::Ad { id: ad_id.clone() },
            )
            .unwrap();

        assert_eq!(ad.views, 1);

        // Delete the ad
        app.execute_contract(
            Addr::unchecked("owner"),
            contract_addr.clone(),
            &ExecuteMsg::DeleteAd { id: ad_id.clone() },
            &[],
        )
        .unwrap();

        // Query all ads to ensure the ad has been deleted
        let ads: QueryAllAdsResponse = app
            .wrap()
            .query_wasm_smart(contract_addr, &QueryMsg::Ads)
            .unwrap();

        assert!(ads.ads.is_empty());
    }
}

