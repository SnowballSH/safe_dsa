/*!
Sorting Algorithms

Heap Sort

Implements:
- [sort]
*/

use crate::sorting::helper::{ge_seg, is_sorted};
use contracts::{debug_ensures, debug_requires};

/// Specification function for heap
fn is_heap<T: Ord>(arr: &[T]) -> bool {
    for i in 1..arr.len() {
        let parent = (i - 1) / 2;
        if arr[parent] < arr[i] {
            return false;
        }
    }
    true
}

/// Restores the invariant of a heap from `start` to `end`
#[debug_requires(start <= end && end < arr.len())]
fn sift_down<T: Ord>(arr: &mut [T], start: usize, end: usize) {
    let mut parent = start;
    let mut child = parent * 2 + 1;
    while child <= end {
        debug_assert_eq!(child / 2, parent);
        // Pick the child with larger value
        if child + 1 <= end && arr[child] < arr[child + 1] {
            child += 1;
        }
        if arr[parent] >= arr[child] {
            return; // we are done
        }
        arr.swap(parent, child);
        parent = child;
        child = parent * 2 + 1;
    }
}

/**
Heap Sort

Given an array `arr` containing `n` elements of type `T`, sort it in-place in nondecreasing order.

`T` must implement [Ord].

Worst-case Time Complexity: `O(n log n)`
Average-case Time Complexity: `O(n log n)`
Best-case Time Complexity: `O(n log n)`
Space Complexity: `O(1)`
Stable?: No
*/
#[debug_ensures(is_sorted(arr))]
pub fn sort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();
    // First heapify arr
    for i in (0..n / 2).rev() {
        sift_down(arr, i, n - 1);
    }

    debug_assert!(is_heap(arr));
    // Specifically, arr[0] is the largest element
    debug_assert!(n == 0 || ge_seg(&arr[0], &arr));

    // Now swap largest with end element and sift down
    for i in (1..n).rev() {
        debug_assert!(is_sorted(&arr[i..]));

        arr.swap(0, i);
        sift_down(arr, 0, i - 1);

        debug_assert!(is_heap(&arr[0..i]));
        debug_assert!(ge_seg(&arr[0], &arr[0..i]));
        // The suffix is now sorted
        debug_assert!(is_sorted(&arr[i - 1..]));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sorting::sort_test_helper::test_sort;

    #[test]
    fn test_heap_sort() {
        test_sort(sort);
    }
}
