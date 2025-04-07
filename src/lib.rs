use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen(start)]
pub fn start() {
    console::log_1(&"Rust WASM game loaded!".into());
}

#[wasm_bindgen]
pub fn clicked_square(score: u32) -> u32 {
    let new_score = score + 1;
    console::log_1(&format!("Pontuação: {}", new_score).into());
    new_score
}
