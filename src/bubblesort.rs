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

use super::Sorter;

pub struct BubbleSort;

impl Sorter for BubbleSort {
  fn sort<T: Ord>(&self, slice: &mut [T]) {
    let mut swapped = true;

    while swapped {
      swapped = false;

      for i in 0..(slice.len() - 1) {
        if slice[i] > slice[i + 1] {
          slice.swap(i, i + 1);
          swapped = true;
        }
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn bubble_sort() {
    // Unsorted vector of numbers.
    let mut unsorted = vec![5, 2, 1, 3, 4];

    // Sort unsorted numbers.
    BubbleSort.sort(&mut unsorted);

    // Check if sorting works.
    assert_eq!(unsorted, &[1, 2, 3, 4, 5]);
  }
}
