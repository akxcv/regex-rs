extern crate cfg_if;
extern crate wasm_bindgen;
extern crate regex;

mod utils;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;
use regex::Regex;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn regex_test(pattern: &str, str: &str) -> bool {
    let re = Regex::new(pattern).unwrap();
    re.is_match(str)
}