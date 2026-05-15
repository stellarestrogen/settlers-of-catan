use wasm_bindgen::{JsError, prelude::wasm_bindgen};

use crate::game::{
    Game,
    edition::{BaseEdition, ExpansionEdition},
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
}
