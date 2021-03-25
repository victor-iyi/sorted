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

/// QuickSort -
#[derive(Debug)]
pub struct QuickSort;

impl QuickSort {
  fn quicksort<T: Ord>(slice: &mut [T]) {
    match slice.len() {
      0 | 1 => return,
      2 => {
        if slice[0] > slice[1] {
          slice.swap(0, 1);
        }
        return;
      }
      _ => {}
    }

    let (pivot, rest) = slice.split_first_mut().expect("slice is non-empty");
    let mut left = 0;
    let mut right = rest.len() - 1;
    while left <= right {
      if &rest[left] <= pivot {
        // already on the correct side.
        left += 1;
      } else if &rest[right] > pivot {
        // right already on the correct side
        // avoid unnecessary swaps back & forth.
        if right == 0 {
          // must be done.
          break;
        }
        right -= 1;
      } else {
        // left holds a right, and right holds a left, swap them.
        rest.swap(left, right);
        left += 1;
        if right == 0 {
          // must be done.
          break;
        }
        right -= 1;
      }
    }

    // re-align left to account for the pivot at 0.
    let left = left + 1;

    // Place the pivot at its final location.
    slice.swap(0, left - 1);

    // `split_at_mut(mid: usize) -> (&mut [..mid], &mut [mid..])`
    let (left, right) = slice.split_at_mut(left - 1);

    // Sort left half recursively.
    Self::quicksort(left);

    // Sort right half recursively.
    Self::quicksort(&mut right[1..]);
  }
}

// Implementation of QuickSort
impl<T: Ord> Sorter<T> for QuickSort {
  fn sort(&self, slice: &mut [T]) {
    // [ unsorted | pivot | unsorted ]
    Self::quicksort(slice)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn quick_sort() {
    // Unsorted vector of numbers.
    let mut numbers = vec![5, 2, 1, 3, 4];

    // Sort unsorted numbers.
    QuickSort.sort(&mut numbers);

    // Check if sorting works.
    assert_eq!(numbers, &[1, 2, 3, 4, 5]);
  }
}
