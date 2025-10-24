/*!
Sorting Algorithms

Merge Sort

Implements:
- [sort]
*/

use crate::sorting::helper::is_sorted;
use contracts::{debug_ensures, debug_requires};

/// Merges sorted arrays `a` and `b` into a single sorted array `result`
#[debug_requires(result.len() >= a.len() + b.len())]
#[debug_requires(is_sorted(a))]
#[debug_requires(is_sorted(b))]
#[debug_ensures(is_sorted(result))]
fn merge<T: Ord + Clone>(a: &[T], b: &[T], result: &mut [T]) {
    let mut i: usize = 0;
    let mut j: usize = 0;
    while i < a.len() || j < b.len() {
        if i == a.len() || (j != b.len() && b[j] < a[i]) {
            result[i + j] = b[j].clone();
            j += 1;
        } else {
            result[i + j] = a[i].clone();
            i += 1;
        }
        debug_assert!(is_sorted(&result[..i + j]));
    }
    debug_assert!(i == a.len() && j == b.len());
}

/// Sorts `arr[l, r)` recursively.
#[debug_requires(l <= r && r <= arr.len())]
#[debug_ensures(is_sorted(&arr[l..r]))]
fn mergesort<T: Ord + Clone>(arr: &mut [T], l: usize, r: usize) {
    if r - l <= 1 {
        return;
    }

    let mid = l + (r - l) / 2;
    mergesort(arr, l, mid);
    debug_assert!(is_sorted(&arr[l..mid]));
    mergesort(arr, mid, r);
    debug_assert!(is_sorted(&arr[mid..r]));
    let mut result = arr[l..r].to_vec();
    merge(&arr[l..mid], &arr[mid..r], &mut result);
    arr[l..r].clone_from_slice(&result);
}

/**
Merge Sort

Given an array `arr` containing `n` elements of type `T`, sort it in-place in nondecreasing order.

`T` must implement [Ord] and [Clone].

Worst-case Time Complexity: `O(n log n)`
Average-case Time Complexity: `O(n log n)`
Best-case Time Complexity: `O(n log n)`
Space Complexity: `O(n)`
Stable?: Yes
*/
#[debug_ensures(is_sorted(arr))]
pub fn sort<T: Ord + Clone>(arr: &mut [T]) {
    mergesort(arr, 0, arr.len());
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sorting::sort_test_helper::test_sort;

    #[test]
    fn test_merge_sort() {
        test_sort(sort);
    }
}
