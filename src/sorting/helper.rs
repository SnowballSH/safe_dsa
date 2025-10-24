/*!
Sorting Algorithms — Helper Functions

These helpers operate on the entire slice you pass in. If you want to check a subarray,
use Rust slicing syntax (e.g., `&arr[lo..hi]`) when calling them.
*/

/// Returns `true` iff `arr` is sorted in nondecreasing order.
pub fn is_sorted<T: Ord>(arr: &[T]) -> bool {
    for i in 1..arr.len() {
        if arr[i - 1] > arr[i] {
            return false;
        }
    }
    true
}

/// Returns whether `x` is strictly greater than every element of `arr`.
pub fn gt_seg<T: Ord>(x: &T, arr: &[T]) -> bool {
    match arr.iter().max() {
        Some(max) => x > max,
        None => true,
    }
}

/// Returns whether `x` is greater than or equal to every element of `arr`.
pub fn ge_seg<T: Ord>(x: &T, arr: &[T]) -> bool {
    match arr.iter().max() {
        Some(max) => x >= max,
        None => true,
    }
}

/// Returns whether `x` is strictly less than every element of `arr`.
pub fn lt_seg<T: Ord>(x: &T, arr: &[T]) -> bool {
    match arr.iter().min() {
        Some(min) => x < min,
        None => true,
    }
}

/// Returns whether `x` is less than or equal to every element of `arr`.
pub fn le_seg<T: Ord>(x: &T, arr: &[T]) -> bool {
    match arr.iter().min() {
        Some(min) => x <= min,
        None => true,
    }
}

/// Returns whether all elements of `arr1` are strictly greater than all elements of `arr2`.
pub fn gt_segs<T: Ord>(arr1: &[T], arr2: &[T]) -> bool {
    let min1 = arr1.iter().min();
    let max2 = arr2.iter().max();
    match (min1, max2) {
        (Some(a), Some(b)) => a > b,
        _ => true,
    }
}

/// Returns whether all elements of `arr1` are greater than or equal to all elements of `arr2`.
pub fn ge_segs<T: Ord>(arr1: &[T], arr2: &[T]) -> bool {
    let min1 = arr1.iter().min();
    let max2 = arr2.iter().max();
    match (min1, max2) {
        (Some(a), Some(b)) => a >= b,
        _ => true,
    }
}

/// Returns whether all elements of `arr1` are strictly less than all elements of `arr2`.
pub fn lt_segs<T: Ord>(arr1: &[T], arr2: &[T]) -> bool {
    let max1 = arr1.iter().max();
    let min2 = arr2.iter().min();
    match (max1, min2) {
        (Some(a), Some(b)) => a < b,
        _ => true,
    }
}

/// Returns whether all elements of `arr1` are less than or equal to all elements of `arr2`.
pub fn le_segs<T: Ord>(arr1: &[T], arr2: &[T]) -> bool {
    let max1 = arr1.iter().max();
    let min2 = arr2.iter().min();
    match (max1, min2) {
        (Some(a), Some(b)) => a <= b,
        _ => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---- is_sorted --------------------------------------------------------

    #[test]
    fn is_sorted_basic_sorted() {
        let arr = vec![1, 2, 3, 4, 5];
        assert!(is_sorted(&arr));
    }

    #[test]
    fn is_sorted_all_equal() {
        let arr = vec![7, 7, 7, 7];
        assert!(is_sorted(&arr));
    }

    #[test]
    fn is_sorted_descending_false() {
        let arr = vec![5, 4, 3, 2, 1];
        assert!(!is_sorted(&arr));
    }

    #[test]
    fn is_sorted_subarray_true_inside_unsorted_array() {
        let arr = vec![9, 2, 2, 3, 1];
        assert!(is_sorted(&arr[1..4]));
        assert!(!is_sorted(&arr[..]));
    }

    #[test]
    fn is_sorted_subarray_false() {
        let arr = vec![1, 3, 2, 4, 5];
        assert!(!is_sorted(&arr[0..3]));
        assert!(is_sorted(&arr[2..5]));
    }

    #[test]
    fn is_sorted_single_element_and_empty_ranges() {
        let arr = vec![10, 2, 3, 4, 5];
        assert!(is_sorted(&arr[0..1]));
        assert!(is_sorted(&arr[3..3]));
        assert!(is_sorted(&arr[5..5]));
    }

    #[test]
    fn is_sorted_bounds_last_two() {
        let arr = vec![1, 2, 3, 4, 5];
        assert!(is_sorted(&arr[3..5]));
    }

    #[test]
    fn is_sorted_strings() {
        let arr = vec!["ant", "bee", "bee", "cat", "dog"];
        assert!(is_sorted(&arr));
        assert!(is_sorted(&arr[1..4]));
        let arr2 = vec!["dog", "cat"];
        assert!(!is_sorted(&arr2));
    }

    // ---- gt_seg -----------------------------------------------------------

    #[test]
    fn gt_seg_greater_simple() {
        let arr = vec![1, 2, 3];
        assert!(gt_seg(&4, &arr));
        assert!(gt_seg(&3, &arr[0..2]));
    }

    #[test]
    fn gt_seg_equal_to_max_is_false() {
        let arr = vec![1, 2, 3, 3];
        assert!(!gt_seg(&3, &arr));
        assert!(!gt_seg(&3, &arr[1..4]));
    }

    #[test]
    fn gt_seg_uses_subarray_only() {
        let arr = vec![100, 1, 2, 3, 1000];
        assert!(gt_seg(&4, &arr[1..4]));
        assert!(!gt_seg(&3, &arr[1..4]));
    }

    #[test]
    fn gt_seg_empty_segment_is_true() {
        let arr = vec![1, 2, 3];
        assert!(gt_seg(&-100, &arr[1..1]));
        assert!(gt_seg(&0, &arr[0..0]));
    }

    #[test]
    fn gt_seg_negative_numbers() {
        let arr = vec![-10, -5, -1];
        assert!(gt_seg(&0, &arr));
        assert!(!gt_seg(&-1, &arr));
    }

    #[test]
    fn gt_seg_strings_non_copy() {
        let arr = vec![
            String::from("alpha"),
            String::from("beta"),
            String::from("delta"),
        ];
        assert!(gt_seg(&String::from("omega"), &arr));
        assert!(!gt_seg(&String::from("delta"), &arr));
        assert!(gt_seg(&String::from("gamma"), &arr[0..2]));
    }

    // ---- ge_seg -----------------------------------------------------------

    #[test]
    fn ge_seg_greater_or_equal_cases() {
        let arr = vec![1, 2, 3, 3];
        assert!(ge_seg(&3, &arr));
        assert!(ge_seg(&4, &arr));
        assert!(!ge_seg(&2, &arr[0..3]));
        assert!(!ge_seg(&0, &arr));
    }

    #[test]
    fn ge_seg_empty_segment_true() {
        let arr = vec![1, 2, 3];
        assert!(ge_seg(&999, &arr[2..2]));
    }

    #[test]
    fn ge_seg_strings_non_copy() {
        let arr = vec![
            String::from("ant"),
            String::from("bee"),
            String::from("cat"),
        ];
        assert!(ge_seg(&String::from("cat"), &arr));
        assert!(!ge_seg(&String::from("ant"), &arr));
    }

    // ---- lt_seg -----------------------------------------------------------

    #[test]
    fn lt_seg_less_cases() {
        let arr = vec![2, 3, 4];
        assert!(lt_seg(&1, &arr));
        assert!(!lt_seg(&2, &arr));
        assert!(lt_seg(&2, &arr[1..3]));
    }

    #[test]
    fn lt_seg_empty_true() {
        let arr = vec![1, 2, 3];
        assert!(lt_seg(&5, &arr[0..0]));
    }

    #[test]
    fn lt_seg_strings_non_copy() {
        let arr = vec![String::from("m"), String::from("n")];
        assert!(lt_seg(&String::from("a"), &arr));
        assert!(!lt_seg(&String::from("m"), &arr));
    }

    // ---- le_seg -----------------------------------------------------------

    #[test]
    fn le_seg_less_or_equal_cases() {
        let arr = vec![1, 2, 3];
        assert!(le_seg(&1, &arr));
        assert!(le_seg(&0, &arr));
        assert!(!le_seg(&2, &arr));
        assert!(le_seg(&2, &arr[1..3]));
    }

    #[test]
    fn le_seg_empty_true() {
        let arr = vec![1, 2, 3];
        assert!(le_seg(&0, &arr[2..2]));
    }

    #[test]
    fn le_seg_strings_non_copy() {
        let arr = vec![String::from("a"), String::from("b"), String::from("c")];
        assert!(le_seg(&String::from("a"), &arr));
        assert!(!le_seg(&String::from("b"), &arr));
    }

    // ---- gt_segs ----------------------------------------------------------

    #[test]
    fn gt_segs_disjoint_true() {
        let arr1 = vec![3, 4, 5];
        let arr2 = vec![0, 1, 2];
        assert!(gt_segs(&arr1, &arr2));
    }

    #[test]
    fn gt_segs_overlapping_false() {
        let arr1 = vec![2, 3, 4];
        let arr2 = vec![3, 4, 5];
        assert!(!gt_segs(&arr1, &arr2));
    }

    #[test]
    fn gt_segs_touching_false() {
        let arr1 = vec![5, 6, 7];
        let arr2 = vec![1, 2, 5];
        assert!(!gt_segs(&arr1, &arr2));
    }

    #[test]
    fn gt_segs_empty_segments_true() {
        let arr1: Vec<i32> = vec![];
        let arr2 = vec![1, 2];
        assert!(gt_segs(&arr1, &arr2));
        let arr3 = vec![10, 20];
        let arr4: Vec<i32> = vec![];
        assert!(gt_segs(&arr3, &arr4));
        assert!(gt_segs(&arr4, &arr1));
    }

    #[test]
    fn gt_segs_subarray_ignored_outside() {
        let arr1 = vec![100, 3, 4, 1000];
        let arr2 = vec![0, 1, 2, 2000];
        assert!(gt_segs(&arr1[1..3], &arr2[0..2]));
        assert!(!gt_segs(&arr1[1..3], &arr2[0..4]));
    }

    #[test]
    fn gt_segs_strings_non_copy() {
        let arr1 = vec![String::from("zoo"), String::from("zzz")];
        let arr2 = vec![String::from("ant"), String::from("yak")];
        assert!(gt_segs(&arr1, &arr2));
    }

    // ---- ge_segs ----------------------------------------------------------

    #[test]
    fn ge_segs_disjoint_and_touching() {
        let arr1 = vec![2, 3];
        let arr2 = vec![1, 2];
        assert!(ge_segs(&arr1, &arr2));
    }

    #[test]
    fn ge_segs_false_when_min_lt_max() {
        let arr1 = vec![1, 2];
        let arr2 = vec![3, 4];
        assert!(!ge_segs(&arr1, &arr2));
    }

    #[test]
    fn ge_segs_empty_vacuous_true() {
        let arr1 = vec![1, 2];
        let arr2: Vec<i32> = vec![];
        assert!(ge_segs(&arr1, &arr2));
        let arr3: Vec<i32> = vec![];
        assert!(ge_segs(&arr3, &arr2));
    }

    #[test]
    fn ge_segs_strings_non_copy() {
        let a1 = vec![String::from("m"), String::from("z")];
        let a2 = vec![String::from("a"), String::from("m")];
        assert!(ge_segs(&a1, &a2));
    }

    // ---- lt_segs ----------------------------------------------------------

    #[test]
    fn lt_segs_disjoint_true() {
        let arr1 = vec![1, 1, 2];
        let arr2 = vec![3, 4, 5];
        assert!(lt_segs(&arr1, &arr2));
    }

    #[test]
    fn lt_segs_overlapping_false() {
        let arr1 = vec![1, 2, 3];
        let arr2 = vec![2, 3, 4];
        assert!(!lt_segs(&arr1, &arr2));
    }

    #[test]
    fn lt_segs_empty_vacuous_true() {
        let arr1: Vec<i32> = vec![];
        let arr2 = vec![1, 2];
        assert!(lt_segs(&arr1, &arr2));
        let arr3 = vec![3, 4];
        let arr4: Vec<i32> = vec![];
        assert!(lt_segs(&arr3, &arr4));
        assert!(lt_segs(&arr4, &arr1));
    }

    #[test]
    fn lt_segs_subarray_checks() {
        let arr1 = vec![0, 1, 2, 100];
        let arr2 = vec![-100, 3, 4, 5];
        assert!(lt_segs(&arr1[1..3], &arr2[1..4]));
        assert!(!lt_segs(&arr1[0..4], &arr2[1..4]));
    }

    #[test]
    fn lt_segs_strings_non_copy() {
        let a1 = vec![String::from("ant"), String::from("bee")];
        let a2 = vec![String::from("cat"), String::from("dog")];
        assert!(lt_segs(&a1, &a2));
    }

    // ---- le_segs ----------------------------------------------------------

    #[test]
    fn le_segs_touching_true() {
        let arr1 = vec![1, 2];
        let arr2 = vec![2, 3];
        assert!(le_segs(&arr1, &arr2));
    }

    #[test]
    fn le_segs_false_when_max_gt_min() {
        let arr1 = vec![3, 4];
        let arr2 = vec![1, 2];
        assert!(!le_segs(&arr1, &arr2));
    }

    #[test]
    fn le_segs_empty_vacuous_true() {
        let arr1 = vec![1, 2];
        let arr2: Vec<i32> = vec![];
        assert!(le_segs(&arr1, &arr2));
        let arr3: Vec<i32> = vec![];
        assert!(le_segs(&arr3, &arr2));
    }

    #[test]
    fn le_segs_subarray_checks() {
        let arr1 = vec![0, 1, 2, 100];
        let arr2 = vec![2, 3, 4, 5];
        assert!(le_segs(&arr1[0..3], &arr2[0..1]));
        assert!(!le_segs(&arr1[0..4], &arr2[0..1]));
    }

    #[test]
    fn le_segs_strings_non_copy() {
        let a1 = vec![String::from("ant"), String::from("bee")];
        let a2 = vec![String::from("bee"), String::from("cat")];
        assert!(le_segs(&a1, &a2));
    }
}
