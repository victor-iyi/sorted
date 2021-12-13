//! Sorted-JS is a library for sorting arrays written in Rust.
use wasm_bindgen::prelude::*;

mod utils;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
  fn alert(s: &str);

  /// Uses the `console.log` function to print a message.
  #[wasm_bindgen(js_namespace = console)]
  fn log(s: &str);
}

/// Alerts a greeting message.
#[wasm_bindgen]
pub fn greet(name: &str) {
  alert(&format!("Hello, {}!", name));
}

/// Adds 5 to the given number.
///
/// Examples:
///
/// ```rust
/// use sorted_js::add_5;
///
/// assert_eq!(add_5(2), 7);
/// assert_eq!(add_5(5), 10);
/// ```
#[wasm_bindgen]
pub fn add_5(value: i32) -> i32 {
  value + 5
}
