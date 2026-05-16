use wasm_bindgen::{JsValue, prelude::wasm_bindgen};

pub mod resource;
pub mod wasm;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    tracing_wasm::set_as_global_default();

    Ok(())
}
