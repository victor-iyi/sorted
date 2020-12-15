use super::*;

/// Use the default`sort` method by type `T`.
#[derive(Debug)]
pub struct StdSorter;

impl Sorter for StdSorter {
  fn sort<T: Ord>(&self, slice: &mut [T]) {
    slice.sort();
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn std_sorter() {
    {
      // Unsorted vector of numbers.
      let mut unsorted = vec![5, 2, 1, 3, 4];

      // Sort unsorted numbers.
      sort(StdSorter, &mut unsorted);

      // Check if sorting works.
      assert_eq!(unsorted, &[1, 2, 3, 4, 5]);
    }

    {
      // Unsorted vector of numbers.
      let mut unsorted = vec![5, 2, 1, 3, 4];

      // Sort unsorted numbers.
      StdSorter.sort(&mut unsorted);

      // Check if sorting works.
      assert_eq!(unsorted, &[1, 2, 3, 4, 5]);
    }
  }
}
