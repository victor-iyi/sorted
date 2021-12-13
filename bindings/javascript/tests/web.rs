// Copyright 2021 Victor I. Afolabi
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//! Test suite for thwe Web and headless browsers.
//!
#![cfg(target_arch = "wasm32")]

use sorted_js as sorted;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_greet() {
  sorted::greet("John Doe");
  sorted::greet("Jane Doe");
  // Runs without error.
  assert!(true);
}

#[wasm_bindgen_test]
fn test_add_5() {
  assert_eq!(sorted::add_5(0), 5);
  assert_eq!(sorted::add_5(2), 7);
  assert_eq!(sorted::add_5(5), 10);
}
