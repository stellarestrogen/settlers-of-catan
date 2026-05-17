use std::num::NonZeroUsize;

use serde::{Deserialize, Serialize};
use tracing::info;
use tsify::Tsify;
use wasm_bindgen::{JsError, prelude::wasm_bindgen};

use crate::{
    distribution::Distribution,
    game::{
        Game,
        edition::{BaseEdition, CustomEdition, ExpansionEdition},
        error::GameError,
    },
    object::{
        TileData,
        resource::{ResourceDistribution, ResourceType},
        structure::OwnedStructures,
        trade::{TradeDistribution, TradeType},
    },
    wasm::resource::WasmTileData,
};

#[wasm_bindgen]
pub struct WasmInterface {
    game: Game,
}

#[wasm_bindgen]
impl WasmInterface {
    pub fn new_base(player_count: usize) -> Result<Self, JsError> {
        let Some(player_count) = NonZeroUsize::new(player_count) else {
            return Err(GameError::InsufficientPlayerCount.into());
        };

        let game = Game::new(BaseEdition, player_count);

        Ok(Self { game })
    }

    pub fn new_expansion(player_count: usize) -> Result<Self, JsError> {
        let Some(player_count) = NonZeroUsize::new(player_count) else {
            return Err(GameError::InsufficientPlayerCount.into());
        };

        Ok(Self {
            game: Game::new(ExpansionEdition, player_count),
        })
    }

    pub fn new_custom(
        edition: <WasmCustomEdition as Tsify>::JsType,
        player_count: usize,
    ) -> Result<Self, JsError> {
        let Some(player_count) = NonZeroUsize::new(player_count) else {
            return Err(GameError::InsufficientPlayerCount.into());
        };

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
            game: Game::new(edition.build(), player_count),
        })
    }

    pub fn get_length(&self) -> u32 {
        self.game.get_board_length()
    }

    pub fn get_width(&self) -> u32 {
        self.game.get_board_width()
    }

    pub fn get_tile_data(&self) -> Vec<<WasmTileData as Tsify>::JsType> {
        let thing = self
            .game
            .get_tile_data()
            .map(WasmTileData::from_tile_data)
            .map(|t| t.into_js().expect(""))
            .collect();
        thing
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