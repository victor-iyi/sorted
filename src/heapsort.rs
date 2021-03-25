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

/// HeapSort -
///
/// This is an inplace heap-sort implementation. The code here is heavily
/// inspired by [Geeks for Geeks article][article].
/// This implementation uses a max-heap to sort the provided slice in
/// ascending order.
///
/// [article]: https://www.geeksforgeeks.org/heap-sort/
#[derive(Debug)]
pub struct HeapSort;

// Implementation of HeapSort
impl<T: Ord> Sorter<T> for HeapSort {
  fn sort(&self, slice: &mut [T]) {
    // Turn `slice` into a heap.
    for i in (0..(slice.len() / 2)).rev() {
      Self::heapify(slice, i);
    }

    // Perform heapsort.
    for unsorted in (0..slice.len()).rev() {
      // We have a valid max heap here, so remove the top.
      slice.swap(0, unsorted);

      // Now we want to make sure that the rest is also sorted.
      Self::heapify(&mut slice[..unsorted], 0);
    }
  }
}

impl HeapSort {
  fn heapify<T: Ord>(slice: &mut [T], root: usize) {
    let mut largest = root;
    let left = 2 * root + 1;
    let right = 2 * root + 2;
    let n = slice.len();

    if left < n && slice[left] > slice[largest] {
      largest = left;
    }
    if right < n && slice[right] > slice[largest] {
      largest = right;
    }
    // ...at this point, `largest` points at the largest of root
    // and its children.
    if largest != root {
      slice.swap(largest, root);
      Self::heapify(slice, largest);
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn arbitrary_sort() {
    // Unsorted vector of numbers.
    let mut numbers = vec![5, 2, 1, 3, 4];

    // Sort unsorted numbers.
    HeapSort.sort(&mut numbers);

    // Check if sorting works.
    assert_eq!(numbers, &[1, 2, 3, 4, 5]);
  }

  #[test]
  fn sorted_array() {
    let mut slice = (1..10).into_iter().collect::<Vec<_>>();

    // Sort sorted array.
    HeapSort.sort(&mut slice);

    assert_eq!(slice, (1..10).into_iter().collect::<Vec<_>>());
  }

  #[test]
  fn simple_edge_cases() {
    // Vector with one element: no sort required.
    let mut one = vec![1];
    HeapSort.sort(&mut one);
    assert_eq!(one, vec![1]);

    // Vector with two elements (sorted).
    let mut sorted_two = vec![1, 2];
    HeapSort.sort(&mut sorted_two);
    assert_eq!(sorted_two, vec![1, 2]);

    // Vector with two elements (unsorted or reversed).
    let mut two = vec![2, 1];
    HeapSort.sort(&mut two);
    assert_eq!(two, vec![1, 2]);

    // Vector with three elements (unsorted).
    let mut three = vec![3, 1, 2];
    HeapSort.sort(&mut three);
    assert_eq!(three, vec![1, 2, 3]);
  }
}
