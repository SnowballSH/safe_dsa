/*!
Sorting Algorithms

Quick Sort

Implements:
- [sort]
*/

use crate::sorting::helper::{ge_seg, is_sorted, le_seg};
use contracts::{debug_ensures, debug_requires};

/// Partitions by `pi`
#[debug_requires(pi <= arr.len())]
#[debug_ensures(ret < arr.len())]
#[debug_ensures(ge_seg(&arr[ret], &arr[..ret]))]
#[debug_ensures(le_seg(&arr[ret], &arr[ret..]))]
fn partition<T: Ord + Clone>(arr: &mut [T], pi: usize) -> usize {
    let pivot = arr[pi].clone();
    arr.swap(0, pi);

    let mut left = 1;
    let mut right = arr.len();

    while left < right {
        debug_assert!(ge_seg(&pivot, &arr[1..left]));
        debug_assert!(le_seg(&pivot, &arr[right..]));

        let ok = arr[left] <= pivot;
        if ok {
            left += 1;
        } else {
            arr.swap(left, right - 1);
            right -= 1;
        }
    }

    debug_assert!(ge_seg(&pivot, &arr[1..left]));
    debug_assert!(le_seg(&pivot, &arr[right..]));
    debug_assert!(left == right);

    arr.swap(0, left - 1);
    left - 1
}

/**
Quick Sort

Given an array `arr` containing `n` elements of type `T`, sort it in-place in nondecreasing order.

`T` must implement [Ord] and [Clone].

Worst-case Time Complexity: `O(n^2)`
Average-case Time Complexity: `O(n log n)`
Best-case Time Complexity: `O(n log n)`
Space Complexity: `O(1)`
Stable?: No
*/
#[debug_ensures(is_sorted(arr))]
pub fn sort<T: Ord + Clone>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    let pi = arr.len() / 2;
    let mid = partition(arr, pi);
    sort(&mut arr[..mid]);
    sort(&mut arr[mid + 1..]);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sorting::sort_test_helper::test_sort;

    #[test]
    fn test_quick_sort() {
        test_sort(sort);
    }
}
