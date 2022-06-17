mod utils;

extern crate js_sys;
extern crate web_sys;

use wasm_bindgen::prelude::*;
use web_sys::console;
use js_sys::{Math, Date};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    // alert("Hello, sign!");
}

#[wasm_bindgen]
pub fn get_random() -> String {
    let mut result = String::from(Date::now().to_string());
    let str = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789");

    let len = str.len() as f64;

    console::log_2(&JsValue::from("len:") ,&JsValue::from(len));

    let mut i = 0;
    while i < 3 {
        let random = Math::floor(Math::random() * len);

        console::log_2(&JsValue::from("random:"),&JsValue::from(random));

        let b = str.chars().nth(random as usize).unwrap().to_string();

        console::log_2(&JsValue::from("b:"),&JsValue::from(b.clone()));

        result.push_str(&*b);
        i += 1;
    }

    result
}
