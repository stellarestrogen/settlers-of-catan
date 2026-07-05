use std::num::NonZeroUsize;

use hexgrid::{
    corner::position::{CornerHeight, CornerPosition},
    hex::position::HexPosition,
};
use serde::Deserialize;
use tsify::Tsify;
use wasm_bindgen::{JsError, prelude::wasm_bindgen};

use crate::{
    distribution::Distribution,
    game::{
        Game,
        edition::{BaseEdition, CustomEdition, ExpansionEdition},
        error::GameError,
    },
    object::{resource::ResourceType, structure::OwnedStructures, trade::TradeType},
    wasm::{
        position::{WasmCornerPosition, WasmHexPosition},
        resource::WasmTileData,
        trade::WasmTradePort,
    },
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

    pub fn get_width(&self) -> u32 {
        self.game.get_board_width()
    }

    pub fn get_height(&self) -> u32 {
        self.game.get_board_height()
    }

    pub fn take_hex_position(&self, position: <WasmHexPosition as Tsify>::JsType) {
        let mut position: HexPosition = position.into();
        position += self.game.get_offset();
        // do something with the resulting position
    }

    pub fn get_tile_data(&self) -> Vec<<WasmTileData as Tsify>::JsType> {
        let tiles = self
            .game
            .get_tile_data()
            .map(WasmTileData::from_tile_data)
            .map(|t| t.into_js().expect(""))
            .collect();
        tiles
    }

    pub fn get_trade_ports(&self) -> Vec<<WasmTradePort as Tsify>::JsType> {
        let trades = self
            .game
            .get_trade_ports()
            .map(WasmTradePort::from_trade_port)
            .map(|t| t.into_js().expect(""))
            .collect();
        trades
    }

    pub fn hex_offset(&self) -> <WasmHexPosition as Tsify>::JsType {
        let position: WasmHexPosition = self.game.get_offset().into();

        position.into_js().expect("")
    }

    pub fn corner_offset(&self) -> <WasmCornerPosition as Tsify>::JsType {
        let position: WasmCornerPosition =
            Into::<CornerPosition>::into(self.game.get_offset() + CornerHeight::TOP_LEFT).into();

        position.into_js().expect("")
    }

    pub fn query_trade(&self, position: <WasmCornerPosition as Tsify>::JsType) {
        let offset = WasmCornerPosition::from_js(self.corner_offset()).expect("");
        let position = WasmCornerPosition::from_js(position).expect("");

        let position = WasmCornerPosition {
            rights: offset.rights + position.rights,
            downs: offset.downs + position.downs,
        };

        let real_position: CornerPosition = position.into();

        if let Some(trade) = self.game.get_trade(real_position) {
            tracing::trace!("The trade here is {:?}\n", trade);
        } else {
            tracing::trace!("There is no trade here.\n")
        }
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
