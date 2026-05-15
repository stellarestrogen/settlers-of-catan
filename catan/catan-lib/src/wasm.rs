use serde::Deserialize;
use tsify::Tsify;
use wasm_bindgen::{JsError, prelude::wasm_bindgen};

use crate::{
    distribution::Distribution, game::{
        Game,
        edition::{BaseEdition, CustomEdition, ExpansionEdition},
    }, object::{
        resource::{ResourceDistribution, ResourceType}, structure::OwnedStructures, trade::{TradeDistribution, TradeType},
    }
};

#[wasm_bindgen]
pub struct WasmInterface {
    game: Game,
}

#[wasm_bindgen]
impl WasmInterface {
    pub fn new_base(player_count: usize) -> Result<Self, JsError> {
        Ok(Self {
            game: Game::new(BaseEdition, player_count)?,
        })
    }

    pub fn new_expansion(player_count: usize) -> Result<Self, JsError> {
        Ok(Self {
            game: Game::new(ExpansionEdition, player_count)?,
        })
    }

    pub fn new_custom(edition: <WasmCustomEdition as Tsify>::JsType, player_count: usize) -> Result<Self, JsError> {
        let wasm = WasmCustomEdition::from_js(edition).expect("Failed to deserialize!");

        let mut edition = CustomEdition::of_size(wasm.shortest, wasm.longest);

        if let Some(resource_distribution) = wasm.resource_distr {
            edition = edition.with_resource_distribution(Distribution::new(resource_distribution));
        }

        if let Some(roll_numbers) = wasm.roll_numbers {
            edition = edition.with_roll_numbers(roll_numbers);
        }

        if let Some(trade_distribution) = wasm.trade_distr {
            edition = edition.with_trade_distribution(Distribution::new(trade_distribution));
        }

        if let Some(trade_gaps) = wasm.trade_gaps {
            edition = edition.with_trade_gaps(trade_gaps);
        }

        if let Some(owned_structures) = wasm.owned_structures {
            edition = edition.with_owned_structures(owned_structures)
        }
        
        Ok(Self {
            game: Game::new(edition.build(), player_count)?,
        })
    }
}

#[derive(Tsify, Deserialize)]
pub struct WasmCustomEdition {
    pub shortest: u32,
    pub longest: u32,
    pub resource_distr: Option<[(ResourceType, u32); 5]>,
    pub roll_numbers: Option<Vec<u8>>,
    pub trade_distr: Option<[(TradeType, u32); 6]>,
    pub trade_gaps: Option<Vec<u32>>,
    pub owned_structures: Option<OwnedStructures>,
}

#[wasm_bindgen]
impl WasmCustomEdition {
    
}

