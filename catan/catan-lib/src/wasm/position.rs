use hexgrid::{
    corner::position::{CornerHeight, CornerPosition},
    edge::position::{EdgeOrientation, EdgePosition},
    hex::position::HexPosition,
};
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, Tsify, Serialize, Deserialize)]
pub struct WasmHexPosition {
    pub rights: i32,
    pub downs: i32,
}

#[wasm_bindgen]
impl WasmHexPosition {
    #[wasm_bindgen(constructor)]
    pub fn new(rights: i32, downs: i32) -> Self {
        Self { rights, downs }
    }
}

impl Into<HexPosition> for WasmHexPosition {
    fn into(self) -> HexPosition {
        let mut new_position = HexPosition::ORIGIN;

        new_position += HexPosition::DOWN_LEFT * (self.downs / 2);
        new_position += HexPosition::DOWN_RIGHT * (self.downs / 2);
        if self.downs.abs() % 2 == 1 {
            new_position += if self.downs.signum() == 1 {
                HexPosition::DOWN_LEFT
            } else if self.downs.signum() == -1 {
                HexPosition::UP_LEFT
            } else {
                HexPosition::ORIGIN
            };
        }

        new_position += HexPosition::RIGHT * self.rights;

        new_position
    }
}

impl Into<WasmHexPosition> for HexPosition {
    fn into(self) -> WasmHexPosition {
        let rights = self.horizontal_displacement(HexPosition::ORIGIN).ceil();
        let downs = self.vertical_displacement(HexPosition::ORIGIN);

        WasmHexPosition { rights, downs }
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, Tsify, Serialize, Deserialize)]
pub struct WasmCornerPosition {
    pub rights: i32,
    pub downs: i32,
}

impl WasmCornerPosition {
    fn structural_owner(&self) -> WasmHexPosition {
        let rights = ((self.rights as f64 + 1.) / 2.).floor() as i32;
        let downs = (if (self.downs % 3).abs() == 0 {
            self.downs + 1
        } else {
            self.downs - 1
        } as f64
            / 3.)
            .floor() as i32;

        WasmHexPosition { rights, downs }
    }
}

#[wasm_bindgen]
impl WasmCornerPosition {
    #[wasm_bindgen(constructor)]
    pub fn new(rights: i32, downs: i32) -> Self {
        Self { rights, downs }
    }

    pub fn neighboring_hex(&self) -> Vec<WasmHexPosition> {
        let corner: CornerPosition = self.clone().into();

        corner
            .neighboring_hex()
            .map(Into::<WasmHexPosition>::into)
            .into()
    }
}

impl Into<CornerPosition> for WasmCornerPosition {
    fn into(self) -> CornerPosition {
        let is_low = (self.downs % 3).abs() == 0;

        let hex: HexPosition = self.structural_owner().into();

        let position = if is_low {
            (hex + CornerHeight::TOP_LEFT).into()
        } else {
            (hex + CornerHeight::BOTTOM_LEFT).into()
        };

        position
    }
}

impl Into<WasmCornerPosition> for CornerPosition {
    fn into(self) -> WasmCornerPosition {
        let origin_corner = (HexPosition::ORIGIN + CornerHeight::TOP_LEFT).into();
        let rights = self.horizontal_distance(origin_corner);
        let downs = self.vertical_distance(origin_corner);

        WasmCornerPosition { rights, downs }
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, Tsify, Serialize, Deserialize)]
pub struct WasmEdgePosition {
    pub rights: i32,
    pub downs: i32,
}

impl WasmEdgePosition {
    fn structural_owner(&self) -> WasmHexPosition {
        let (rights, downs) = if self.is_even() {
            (self.rights + 1, self.downs + 1)
        } else if self.is_odd() {
            (self.rights + 1, self.downs - 1)
        } else {
            (self.rights + 2, self.downs)
        };

        let rights = if rights % 4 == 1 && rights % 4 == -3 {
            (rights - 1) / 4
        } else {
            (rights + 1) / 4
        };

        let hex = WasmHexPosition {
            rights,
            downs: (downs - 1) / 2,
        };

        hex
    }
}

#[wasm_bindgen]
impl WasmEdgePosition {
    #[wasm_bindgen(constructor)]
    pub fn new(rights: i32, downs: i32) -> Self {
        Self { rights, downs }
    }

    pub fn neighboring_hex(&self) -> Vec<WasmHexPosition> {
        let edge: EdgePosition = self.clone().into();
        edge.neighboring_hex()
            .map(Into::<WasmHexPosition>::into)
            .into()
    }

    pub fn is_even(&self) -> bool {
        ((self.rights + self.downs) % 4).abs() == 0 && (self.downs % 2).abs() == 0
    }

    pub fn is_odd(&self) -> bool {
        ((self.rights + self.downs) % 4).abs() == 2 && (self.downs % 2).abs() == 0
    }

    pub fn is_positive(&self) -> bool {
        (self.downs % 2).abs() == 1 && ((self.rights + self.downs) % 4).abs() == 0
    }

    pub fn is_invalid(&self) -> bool {
        !self.is_even() && !self.is_odd() && !self.is_positive()
    }
}

impl Into<EdgePosition> for WasmEdgePosition {
    fn into(self) -> EdgePosition {
        let hex: HexPosition = self.structural_owner().into();
        if self.is_even() {
            (hex + EdgeOrientation::TOP_LEFT).into()
        } else if self.is_odd() {
            (hex + EdgeOrientation::BOTTOM_LEFT).into()
        } else if self.is_positive() {
            (hex + EdgeOrientation::LEFT).into()
        } else {
            unreachable!()
        }
    }
}
