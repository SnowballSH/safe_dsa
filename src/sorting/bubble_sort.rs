/*!
Sorting Algorithms

Bubble Sort

Implements:
- [sort]
*/

use crate::sorting::helper::is_sorted;
use contracts::debug_ensures;

/**
Bubble Sort

Given an array `arr` containing `n` elements of type `T`, sort it in-place in nondecreasing order.

`T` must implement [Ord].

Worst-case Time Complexity: `O(n^2)`
Average-case Time Complexity: `O(n^2)`
Best-case Time Complexity: `O(n)`
Space Complexity: `O(1)`
Stable?: Yes
*/
#[debug_ensures(is_sorted(arr))]
pub fn sort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();
    let mut flag = true;
    for cnt in 1..=n {
        flag = false;
        for i in 1..n {
            if !(arr[i - 1] <= arr[i]) {
                flag = true;
                arr.swap(i - 1, i);
            }
        }
        if !flag {
            break;
        }

        // Loop Invariants
        // The last cnt elements are sorted.
        debug_assert!(is_sorted(&arr[n - cnt..]));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sorting::sort_test_helper::test_sort;

    #[test]
    fn test_bubble_sort() {
        test_sort(sort);
    }
}
