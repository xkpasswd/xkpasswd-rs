//#![feature(test)]

mod utils;
mod xkpasswd;

use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn gen_passwd(count: u8) -> String {
    utils::set_panic_hook();
    xkpasswd::gen_passwd(count)
}
