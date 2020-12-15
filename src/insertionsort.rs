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

/// Insertion sort is a simple sorting algorithm that builds the final sorted array (or list) one item at a time.
///
/// Insertion sort is one of the fastest algorithms for sorting very small arrays, even faster than quicksort;
/// indeed, good quicksort implementations use insertion sort for arrays smaller than a certain threshold,
/// also when arising as subproblems; the exact threshold must be determined experimentally and depends on the machine,
/// but is commonly around ten.
///
/// ## How it works
///
/// Insertion sort iterates, consuming one input element each repetition, and growing a sorted output list.
/// At each iteration, insertion sort removes one element from the input data, finds the location it belongs
/// within the sorted list, and inserts it there. It repeats until no input elements remain.
///
/// Sorting is typically done in-place, by iterating up the array, growing the sorted list behind it.
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
/// The best case input is an array that is already sorted. In this case insertion sort has a linear running time `O(n)`.
/// During each iteration, the first remaining element of the input is only compared with the right-most element of the sorted subsection of the array.
///
/// The simplest worst case input is an array sorted in reverse order. The set of all worst case inputs consists of all arrays where each element is the smallest or second-smallest of the elements before it. In these cases every iteration of the inner loop will scan and shift the entire sorted subsection of the array before inserting the next element. This gives insertion sort a quadratic running time `O(n^2)`.
///
/// The average case is also quadratic, which makes insertion sort impractical for sorting large arrays. However, insertion sort is one of the fastest algorithms for sorting very small arrays, even faster than quicksort; indeed, good quicksort implementations use insertion sort for arrays smaller than a certain threshold, also when arising as subproblems; the exact threshold must be determined experimentally and depends on the machine, but is commonly around ten.
///
/// ## Step-by-step example
///
/// The following table shows the steps for sorting the sequence `[3, 7, 4, 9, 5, 2, 6, 1]`.
/// In each step, the key under consideration is underlined. The key that was moved (or left in place
/// because it was biggest yet considered) in the previous step is marked with an asterisk.
///
/// ```txt
///  3  7  4  9  5  2  6  1
///  3  7* 4  9  5  2  6  1
///  3* 7  4  9  5  2  6  1
///  3  4* 7  9  5  2  6  1
///  3  4  7  9* 5  2  6  1
///  3  4  5* 7  9  2  6  1
///  2* 3  4  5  7  9  6  1
///  2  3  4  5  6* 7  9  1
///  1* 2  3  4  5  6  7  9
///```
#[derive(Debug)]
pub struct InsertionSort {
  smart: bool,
}

// Implementation of InsertionSort
impl Sorter for InsertionSort {
  fn sort<T: Ord>(&self, slice: &mut [T]) {
    for unsorted in 1..slice.len() {
      // [ sorted | not sorted ]
      // slice[unsorted..] is not sorted
      // take slice[unsorted] and place in sorted location in slice[..=unsorted]
      // [ 1 3 4 | 2 ]
      // [ 1 3 4 2 |]
      // [ 1 3 2 4 |]
      // [ 1 2 3 4 |]
      if self.smart {
        let mut i = unsorted;
        while i > 0 && slice[i - 1] > slice[i] {
          slice.swap(i - 1, i);
          i -= 1;
        }
      } else {
        // use binary search to find index.
        // then use .rotate_right to splice in i
        let i = match slice[..unsorted].binary_search(&slice[unsorted]) {
          //[ a, c, e ].binary_search(c) => Ok(1)
          Ok(i) => i,
          // where it should be if 'b' existed in the slice.
          //[ a, c, e ].binary_search(b) => Ok(1)
          Err(i) => i,
        };
        slice[i..=unsorted].rotate_right(1);
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn insertion_sort_without_smart() {
    // Unsorted vector of numbers.
    let mut unsorted = vec![5, 2, 1, 3, 4];

    // Sort unsorted numbers.
    InsertionSort { smart: false }.sort(&mut unsorted);

    // Check if sorting works.
    assert_eq!(unsorted, &[1, 2, 3, 4, 5]);
  }

  #[test]
  fn insertion_sort_with_smart() {
    // Unsorted vector of numbers.
    let mut unsorted = vec![5, 2, 1, 3, 4];

    // Sort unsorted numbers.
    InsertionSort { smart: true }.sort(&mut unsorted);

    // Check if sorting works.
    assert_eq!(unsorted, &[1, 2, 3, 4, 5]);
  }
}
