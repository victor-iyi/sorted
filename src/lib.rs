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
mod insertionsort;
mod selectionsort;

/// Base `sorted` trait which is implemented by all in built and user-defined sorting algorithms.0
pub trait Sorter {
    fn sort<T: Ord>(&self, slice: &mut [T]);
}

/// Generic function to sort a given slice.
pub fn sort<S: Sorter, T: Ord>(obj: S, slice: &mut [T]) {
    obj.sort(slice);
}

#[cfg(test)]
mod tests {
    use super::*;

    struct StdSorter;

    impl Sorter for StdSorter {
        fn sort<T: Ord>(&self, slice: &mut [T]) {
            slice.sort();
        }
    }

    #[test]
    fn std_sorter() {
        // Unsorted vector of numbers.
        let mut unsorted = vec![5, 2, 1, 3, 4];

        // Sort unsorted numbers.
        sort(StdSorter, &mut unsorted);

        // Check if sorting works.
        assert_eq!(unsorted, &[1, 2, 3, 4, 5]);
    }
}
