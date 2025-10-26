/*!
Sorting Algorithms

Shell Sort with gap sequence A003462

Implements:
- [sort]
*/

use crate::sorting::helper::is_sorted;
use contracts::debug_ensures;

/**
Shell Sort with gap sequence A003462

Given an array `arr` containing `n` elements of type `T`, sort it in-place in nondecreasing order.

`T` must implement [Ord, Clone].

Worst-case Time Complexity: `O(n^1.5)`
Average-case Time Complexity: `O(n^1.5)`
Best-case Time Complexity: `O(n log n)`
Space Complexity: `O(1)`
Stable?: No
*/
#[debug_ensures(is_sorted(arr))]
pub fn sort<T: Ord + Clone>(arr: &mut [T]) {
    let n = arr.len();
    let mut k = 1;
    while k < n / 3 {
        k = 3 * k + 1;
    }

    while k >= 1 {
        for i in k..n {
            let tmp = arr[i].clone();
            let mut j = i;
            while j >= k && arr[j - k] > tmp {
                arr[j] = arr[j - k].clone();
                j -= k;
            }
            arr[j] = tmp;
        }
        k /= 3;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sorting::sort_test_helper::test_sort;

    #[test]
    fn test_shell_a003462_sort() {
        test_sort(sort);
    }
}
