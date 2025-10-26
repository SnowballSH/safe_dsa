/*!
Sorting Algorithms

Insertion Sort

Implements:
- [sort]
*/

use crate::sorting::helper::is_sorted;
use contracts::debug_ensures;

/**
Insertion Sort

Given an array `arr` containing `n` elements of type `T`, sort it in-place in nondecreasing order.

`T` must implement [Ord] and [Clone].

Worst-case Time Complexity: `O(n^2)`
Average-case Time Complexity: `O(n^2)`
Best-case Time Complexity: `O(n)`
Space Complexity: `O(1)`
Stable?: Yes
*/
#[debug_ensures(is_sorted(arr))]
pub fn sort<T: Ord + Clone>(arr: &mut [T]) {
    let n = arr.len();
    for i in 1..n {
        debug_assert!(is_sorted(&arr[0..i]));
        let key = arr[i].clone();
        let mut j = i;
        while j > 0 && arr[j - 1] > key {
            arr[j] = arr[j - 1].clone();
            j -= 1;
        }
        arr[j] = key;
        debug_assert!(is_sorted(&arr[0..=i]));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sorting::sort_test_helper::test_sort;

    #[test]
    fn test_insertion_sort() {
        test_sort(sort);
    }
}
