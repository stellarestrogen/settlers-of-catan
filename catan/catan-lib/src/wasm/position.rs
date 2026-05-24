use hexgrid::hex::position::HexPosition;
use serde::Deserialize;
use tsify::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug, Clone, Copy, Tsify, Deserialize)]
pub struct WasmHexPosition {
    pub rights: i32,
    pub downs: i32,
}

impl Into<HexPosition> for <WasmHexPosition as Tsify>::JsType {
    fn into(self) -> HexPosition {
        let position = WasmHexPosition::from_js(self).expect("");

        let mut new_position = HexPosition::ORIGIN;

        new_position += HexPosition::DOWN_LEFT * (position.downs / 2);
        new_position += HexPosition::DOWN_RIGHT * (position.downs / 2);
        if position.downs.abs() % 2 == 1 {
            new_position += HexPosition::DOWN_LEFT;
        }

        new_position += HexPosition::RIGHT * position.rights;

        new_position
    }
}
