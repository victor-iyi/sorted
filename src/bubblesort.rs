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

/// Bubble sort is a simple sorting algorithm and should be avoided in the case of large collections.
/// It will not be efficient in the case of a reverse-ordered collection.
///
/// ## How it works
///
/// Bubble sort, sometimes referred to as sinking sort, is a simple sorting algorithm that
/// repeatedly steps through the list, compares adjacent elements and swaps them if they are
/// in the wrong order. The pass through the list is repeated until the list is sorted.
/// The algorithm, which is a comparison sort, is named for the way smaller or larger
/// elements "bubble" to the top of the list.
///
/// ## Performance `O(n^2)`
///
/// Bubble sort has a worst-case and average complexity of `О(n2)`,
/// where `n` is the number of items being sorted.
///
/// Worst-case:    `O(n^2)`
///
/// Average-case:  `O(n^2)`
///
/// Best-Case:     `O(n)` -- when the list is already sorted.
///
/// ## Step-by-step example
///
/// Take an array of numbers `[5, 1, 4, 2, 8]`, and sort the array from lowest number to greatest
/// number using bubble sort. In each step, elements written in bold are being compared.
///
/// Three passes will be required;
///
/// ```txt
/// First Pass
///
/// ( 5 1 4 2 8 ) → ( 1 5 4 2 8 ), Here, algorithm compares the first two elements, and swaps since 5 > 1.
/// ( 1 5 4 2 8 ) → ( 1 4 5 2 8 ), Swap since 5 > 4
/// ( 1 4 5 2 8 ) → ( 1 4 2 5 8 ), Swap since 5 > 2
/// ( 1 4 2 5 8 ) → ( 1 4 2 5 8 ), Now, since these elements are already in order (8 > 5), algorithm does not swap them.
///
/// Second Pass
///
/// ( 1 4 2 5 8 ) → ( 1 4 2 5 8 )
/// ( 1 4 2 5 8 ) → ( 1 2 4 5 8 ), Swap since 4 > 2
/// ( 1 2 4 5 8 ) → ( 1 2 4 5 8 )
/// ( 1 2 4 5 8 ) → ( 1 2 4 5 8 )
/// Now, the array is already sorted, but the algorithm does not know if it is completed.
/// The algorithm needs one whole pass without any swap to know it is sorted.
///
/// Third Pass
///
/// ( 1 2 4 5 8 ) → ( 1 2 4 5 8 )
/// ( 1 2 4 5 8 ) → ( 1 2 4 5 8 )
/// ( 1 2 4 5 8 ) → ( 1 2 4 5 8 )
/// ( 1 2 4 5 8 ) → ( 1 2 4 5 8 )
///```
///
pub struct BubbleSort;

// Implementation of BubbleSort
impl Sorter for BubbleSort {
  fn sort<T: Ord>(&self, slice: &mut [T]) {
    let mut swapped = true;

    // Loop till we are no longer swapping.
    while swapped {
      // If swapped stay false, we are done sorting.
      swapped = false;

      for i in 1..slice.len() {
        if slice[i - 1] > slice[i] {
          slice.swap(i - 1, i);
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
