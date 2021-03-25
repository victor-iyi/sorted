use super::*;

/// Use the default `sort` method given a type T.
#[derive(Debug)]
pub struct StdSorter;

impl<T: Ord> Sorter<T> for StdSorter {
  fn sort(&self, slice: &mut [T]) {
    slice.sort();
  }
}

/// Use the slice `sort_unstable` method given a type T.
pub struct StdUnstableSorter;

impl<T: Ord> Sorter<T> for StdUnstableSorter {
  fn sort(&self, slice: &mut [T]) {
    slice.sort_unstable();
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn std_sorter() {
    // List of unsorted numbers.
    let mut numbers = vec![5, 3, 1, 2, 4];
    StdSorter.sort(&mut numbers);

    assert_eq!(numbers, &[1, 2, 3, 4, 5]);
  }

  #[test]
  fn std_unstable_sorter() {
    let mut numbers = vec![5, 3, 1, 2, 4];
    StdUnstableSorter.sort(&mut numbers);

    assert_eq!(numbers, &[1, 2, 3, 4, 5]);
  }
}
