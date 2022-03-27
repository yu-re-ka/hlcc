use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn compute_input(s: &str) -> String{
    hlcc_parser::compute(s).unwrap_or_else(|| "Error".to_string())
}
