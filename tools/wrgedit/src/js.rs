#![allow(non_snake_case)]
pub use warg::js::api::init_once;
use warg::js::api::GameHandle;
use warg::js::runner::JSGameRunner;
use wasm_bindgen::prelude::*;

use crate::WrgEditor;

#[wasm_bindgen]
pub fn WrgEditorHandle() -> GameHandle {
    JSGameRunner::new(WrgEditor::new(), Some(30.0), Some(30.0)).handle()
}
