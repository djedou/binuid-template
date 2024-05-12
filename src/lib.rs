use wasm_bindgen::prelude::*;
use binuid_app::binuid_vm::{init_trace, console::info, engine::BinuidEngine, vm::{binuid_instructions_table, WriteManyTable, Operand}};

#[wasm_bindgen]
pub fn binuid_engine() {
   init_trace();
   let constants: WriteManyTable<Operand> = WriteManyTable::new();
   let _ = BinuidEngine::start(&constants, &binuid_instructions_table());
   info!("Binuid Engine Ended !!!!!");
}
