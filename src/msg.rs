use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Initialization message
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {}

/// Response for querying total views.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TotalViewsResponse {
    pub total_views: u64,
}

/// Response for querying a single ad.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct QueryAdResponse {
    pub id: String,
    pub image_url: String,
    pub target_url: String,
    pub views: u64,
    pub reward_address: String,
}

/// Response for querying all ads.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct QueryAllAdsResponse {
    pub ads: Vec<QueryAdResponse>,
}

/// Represents a single advertisement entry in state.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Ad {
    pub id: String,
    pub image_url: String,
    pub target_url: String,
    pub views: u64,
    pub reward_address: String,
}

/// Execution messages to modify the state of the contract.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    /// Add a new ad with given details.
    AddAd {
        id: String,
        image_url: String,
        target_url: String,
        reward_address: String,
    },
    /// Increment the view count of the given ad.
    ServeAd {
        id: String,
    },
    /// Remove an ad by its ID.
    DeleteAd {
        id: String,
    },
    /// Serve multiple ads in a single transaction.
    BatchServeAds {
        ids: Vec<String>,
    },
}

/// Query messages to read state.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// Query a specific ad by ID.
    Ad { id: String },
    /// Query all ads.
    Ads,
    /// Query total views across all ads.
    TotalViews,
}

