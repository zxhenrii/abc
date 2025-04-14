use wasm_bindgen::prelude::*;
use web_sys::{console, window};
use js_sys::Math;
use std::cell::RefCell;
use std::rc::Rc;

static mut SCORE: u32 = 0;
static mut HIGH_SCORE: u32 = 0;
static mut TIME_LEFT: i32 = 30;

#[wasm_bindgen(start)]
pub fn start() {
    console::log_1(&"Rust WASM game loaded!".into());
    spawn_timer();
}

fn spawn_timer() {
    let closure = Closure::wrap(Box::new(move || {
        unsafe {
            TIME_LEFT -= 1;
            update_time_display(TIME_LEFT);
            if TIME_LEFT <= 0 {
                game_over();
            }
        }
    }) as Box<dyn FnMut()>);

    window()
        .unwrap()
        .set_interval_with_callback_and_timeout_and_arguments_0(closure.as_ref().unchecked_ref(), 1000)
        .unwrap();

    closure.forget();
}

fn update_time_display(seconds: i32) {
    let document = window().unwrap().document().unwrap();
    if let Some(elem) = document.get_element_by_id("timer") {
        elem.set_inner_html(&format!("Time Left: {}", seconds));
    }
}

fn game_over() {
    let document = window().unwrap().document().unwrap();
    if let Some(elem) = document.get_element_by_id("game-area") {
        elem.set_inner_html("<h2>Game Over</h2><p>Reload to play again!</p>");
    }
}

#[wasm_bindgen]
pub fn clicked_square() -> u32 {
    unsafe {
        let bonus = if Math::random() < 0.2 { 3 } else { 1 };
        SCORE += bonus;
        if SCORE > HIGH_SCORE {
            HIGH_SCORE = SCORE;
        }
        update_score_display(SCORE, HIGH_SCORE);
        SCORE
    }
}

fn update_score_display(score: u32, high_score: u32) {
    let document = window().unwrap().document().unwrap();
    if let Some(elem) = document.get_element_by_id("score") {
        elem.set_inner_html(&format!("Score: {} | High Score: {}", score, high_score));
    }
}
