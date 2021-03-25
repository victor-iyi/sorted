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
#[derive(Debug)]
pub struct SelectionSort;

// Implementation of SelectionSort
impl<T: Ord> Sorter<T> for SelectionSort {
  fn sort(&self, slice: &mut [T]) {
    for unsorted in 0..slice.len() {
      // Continually check for smallest element in the remaining unsorted slice.
      let smallest_in_rest = slice[unsorted..]
        .iter()
        .enumerate()
        .min_by_key(|&(_, v)| v)
        .map(|(i, _)| unsorted + i)
        .expect("Slice is non-empty.");

      // let mut smallest_in_rest = unsorted;
      // for i in (unsorted + 1)..slice.len() {
      //   // Check for the smallest element in the remainder.
      //   if slice[i] < slice[smallest_in_rest] {
      //     smallest_in_rest = i;
      //   }
      // }
      if unsorted != smallest_in_rest {
        slice.swap(unsorted, smallest_in_rest);
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn selection_sort() {
    // Unsorted vector of numbers.
    let mut numbers = vec![5, 2, 1, 3, 4];

    // Sort unsorted numbers.
    SelectionSort.sort(&mut numbers);

    // Check if sorting works.
    assert_eq!(numbers, &[1, 2, 3, 4, 5]);
  }
}
