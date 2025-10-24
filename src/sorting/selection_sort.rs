/*!
Sorting Algorithms

Selection Sort

Implements:
- [sort]
*/

use crate::sorting::helper::{is_sorted, le_seg, le_segs};
use contracts::debug_ensures;

/**
Selection Sort

Given an array `arr` containing `n` elements of type `T`, sort it in-place in nondecreasing order.

`T` must implement [Ord].

Worst-case Time Complexity: `O(n^2)`
Average-case Time Complexity: `O(n^2)`
Best-case Time Complexity: `O(n^2)`
Space Complexity: `O(1)`
Stable?: No
*/
#[debug_ensures(is_sorted(arr))]
pub fn sort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();
    for i in 0..n {
        // k is the index of a minimal element in [i+1, n)
        let mut k = i;
        for j in (i + 1)..n {
            if arr[j] < arr[k] {
                k = j;
            }

            // Loop Invariants
            // debug_assert!(le_seg(&arr[k], &arr[i..=j]))
        }
        arr.swap(i, k);

        // Loop Invariants
        debug_assert!(is_sorted(&arr[..=i]));
        debug_assert!(le_segs(&arr[..=i], &arr[i + 1..]));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sorting::sort_test_helper::test_sort;

    #[test]
    fn test_selection_sort() {
        test_sort(sort);
    }
}
