#[cfg(feature = "wasm_dev")]
extern crate web_sys;

macro_rules! console_log {
    ( $( $t:tt )* ) => {
        #[cfg(feature = "wasm_dev")]
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

pub fn set_panic_hook() {
    #[cfg(feature = "wasm_dev")]
    console_error_panic_hook::set_once();
}

pub(crate) use console_log;
