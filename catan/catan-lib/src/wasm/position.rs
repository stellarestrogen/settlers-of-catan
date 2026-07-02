use hexgrid::{
    corner::position::{CornerHeight, CornerPosition},
    edge::position::{EdgeOrientation, EdgePosition},
    hex::position::HexPosition,
};
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

        position.into()
    }
}

impl Into<HexPosition> for WasmHexPosition {
    fn into(self) -> HexPosition {
        let mut new_position = HexPosition::ORIGIN;

        new_position += HexPosition::DOWN_LEFT * (self.downs / 2);
        new_position += HexPosition::DOWN_RIGHT * (self.downs / 2);
        if self.downs.abs() % 2 == 1 {
            new_position += HexPosition::DOWN_LEFT;
        }

        new_position += HexPosition::RIGHT * self.rights;

        new_position
    }
}

#[derive(Debug, Clone, Copy, Tsify, Deserialize)]
pub struct WasmCornerPosition {
    pub rights: i32,
    pub downs: i32,
}

impl WasmCornerPosition {
    fn structural_owner(&self) -> WasmHexPosition {
        let rights = self.rights + 1;
        let downs = if (self.downs % 3).abs() == 0 {
            self.downs + 1
        } else {
            self.downs - 1
        };

        WasmHexPosition { rights, downs }
    }
}

impl Into<CornerPosition> for <WasmCornerPosition as Tsify>::JsType {
    fn into(self) -> CornerPosition {
        let position = WasmCornerPosition::from_js(self).expect("");

        let is_low = (position.downs % 3).abs() == 0;

        let hex: HexPosition = position.structural_owner().into();

        if is_low {
            (hex + CornerHeight::TOP_LEFT).into()
        } else {
            (hex + CornerHeight::BOTTOM_LEFT).into()
        }
    }
}

#[derive(Debug, Clone, Copy, Tsify, Deserialize)]
pub struct WasmEdgePosition {
    rights: i32,
    downs: i32,
}

impl WasmEdgePosition {
    fn is_even(&self) -> bool {
        (self.downs % 4).abs() == 0
    }

    fn is_odd(&self) -> bool {
        (self.downs % 4).abs() == 2
    }

    fn is_positive(&self) -> bool {
        !self.is_even() && !self.is_odd()
    }

    fn structural_owner(&self) -> WasmHexPosition {
        let (rights, downs) = if self.is_even() {
            (self.rights + 1, self.downs + 1)
        } else if self.is_odd() {
            (self.rights + 1, self.downs - 1)
        } else {
            (self.rights + 2, self.downs)
        };

        WasmHexPosition {
            rights: (rights - 1) / 4,
            downs: (downs - 1) / 2,
        }
    }
}

impl Into<EdgePosition> for <WasmEdgePosition as Tsify>::JsType {
    fn into(self) -> EdgePosition {
        let position = WasmEdgePosition::from_js(self).expect("");

        let hex: HexPosition = position.structural_owner().into();

        if position.is_even() {
            (hex + EdgeOrientation::TOP_LEFT).into()
        } else if position.is_odd() {
            (hex + EdgeOrientation::BOTTOM_LEFT).into()
        } else {
            (hex + EdgeOrientation::LEFT).into()
        }
    }
}
