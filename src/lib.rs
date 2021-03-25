#![warn(rust_2018_idioms, missing_debug_implementations)]

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

mod bubblesort;
mod heapsort;
mod insertionsort;
mod quicksort;
mod selectionsort;
mod stdsorter;

/// Base `sorted` trait which is implemented by all in built and user-defined sorting algorithms.0
pub trait Sorter<T: Ord> {
  /// All sorting algorithm must implement `sort`.
  fn sort(&self, slice: &mut [T]);
}

pub use bubblesort::BubbleSort;
pub use heapsort::HeapSort;
pub use insertionsort::InsertionSort;
pub use quicksort::QuickSort;
pub use selectionsort::SelectionSort;
pub use stdsorter::StdSorter;
