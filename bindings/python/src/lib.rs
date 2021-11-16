// Copyright 2020 Victor I. Afolabi
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

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
fn add_5(value: i32) -> i32 {
  value + 5
}

// #[pyfunction]
// fn std_sorter<T>(slice: &mut [T])
// where
//   T: Ord,
// {
//   slice.sort();
// }
//
// enum SortAlgorithm {
//   Bubble(st::bubblesort::BubbleSort),
//   Heap,
//   Insertion(st::insertionsort::InsertionSort),
//   Quick,
//   Selection(st::selectionsort::SelectionSort),
//   StdSorter(st::stdsorter::StdSorter),
// }
//
// impl Default for SortAlgorithm {
//   fn default() -> SortAlgorithm {
//     SortAlgorithm::StdSorter
//   }
// }
//
// fn sort<T: std::cmp::Ord>(value: &mut [T]) {
//   // Sort with the default sort algorithm.
//   sort_with(value, SortAlgorithm::default())
// }
//
// fn sort_with<T: std::cmp::Ord>(value: &mut [T], algorithm: SortAlgorithm) {
//   // Choose sort algorithm.
//   match algorithm {
//     SortAlgorithm::Bubble(s) => s.sort(&mut value),
//     SortAlgorithm::Insertion(s) => s.sort(&mut value),
//     SortAlgorithm::Selection(s) => s.sort(&mut value),
//     SortAlgorithm::StdSorter => s.sort(&mut value),
//     SortAlgorithm::Quick => {}
//     SortAlgorithm::Heap => {}
//   }
//
//   // algorithm.sort(&mut value);
// }

/// Sorted module provides a set of functions for sorting various collections.
///
/// # List of Sorting Algorithms
///
/// - `bubble_sort`: Worst case: `O(n^2)` | Best case: `O(n)`
/// - `insertion_sort`: Worst case: `O(n^2)` | Best case: `O(n)`
/// - `selection_sort`: Worst case: `O(n^2)` | Best case: `O(n)`
/// - `std_sorter`: Worst case: `O(n^2)` | Best case: `O(n)`
/// - `heap_sort`: Worst case: `O(n^2)` | Best case: `O(n)`
/// - `quick_sort`: Worst case: `O(n^2)` | Best case: `O(n)`
///
#[pymodule]
fn sorted(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
  m.add_function(wrap_pyfunction!(add_5, m)?)?;
  Ok(())
}
