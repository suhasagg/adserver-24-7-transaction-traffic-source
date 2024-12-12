use cosmwasm_std::StdResult;
use cw_storage_plus::Item;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::msg::Ad;

/// The top-level state of the contract.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub ads: Vec<Ad>,
    pub total_views: u64,
    pub plt_address: String,
}

/// Storage item that holds the entire contract state.
pub const STATE: Item<State> = Item::new("state");

/// Loads the current state from storage.
pub fn load_state(storage: &dyn cosmwasm_std::Storage) -> StdResult<State> {
    STATE.load(storage)
}

/// Saves the given state back to storage.
pub fn save_state(storage: &mut dyn cosmwasm_std::Storage, state: &State) -> StdResult<()> {
    STATE.save(storage, state)
}

