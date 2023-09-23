mod engine;

pub use self::engine::BinuidEngine;
use wasm_bindgen::prelude::*;
use binuid_app::binuid_vm::{init_trace, console::info};

#[wasm_bindgen]
pub fn binuid_engine() {
   init_trace();
   
   let _ = BinuidEngine::start();
   info!("Binuid Engine Ended !!!!!");
}
