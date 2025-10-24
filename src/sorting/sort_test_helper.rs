#![cfg(test)]
type IntegerSortingAlgorithm = fn(arr: &mut [isize]);

// ---- Internal helpers ----

fn run_and_compare_full(sort: IntegerSortingAlgorithm, arr: &mut [isize]) {
    let original = arr.to_vec();

    // Expected result using the standard library's sort (deterministic).
    let mut expected = original.clone();
    expected.sort();

    // Run the user's sort.
    sort(arr);

    // Check: equal to std sort, nondecreasing, and idempotent.
    assert_eq!(
        arr,
        &expected[..],
        "Sorting failed.\n  input:  {:?}\n  got:    {:?}\n  expect: {:?}",
        original,
        arr,
        expected
    );
    assert!(
        arr.windows(2).all(|w| w[0] <= w[1]),
        "Array not in nondecreasing order after sort: {:?}",
        arr
    );

    // Idempotency: sorting an already-sorted slice should not change it.
    let second = {
        let mut tmp = arr.to_vec();
        sort(&mut tmp);
        tmp
    };
    assert_eq!(
        second, expected,
        "Sorting not idempotent.\n  once:  {:?}\n  twice: {:?}",
        expected, second
    );
}

fn run_and_compare_partial(sort: IntegerSortingAlgorithm, arr: &mut [isize], lo: usize, hi: usize) {
    assert!(lo <= hi && hi <= arr.len(), "invalid subslice [{lo}, {hi})");

    let original = arr.to_vec();

    // Build expected by sorting only the same subrange.
    let mut expected = original.clone();
    expected[lo..hi].sort();

    // Sort only the subslice
    sort(&mut arr[lo..hi]);

    // Everything outside [lo,hi) should remain identical to the original.
    assert_eq!(
        &arr[..lo],
        &original[..lo],
        "Prefix outside the sorted subslice was modified"
    );
    assert_eq!(
        &arr[hi..],
        &original[hi..],
        "Suffix outside the sorted subslice was modified"
    );

    // The whole array should match our expected (subslice-sorted) version.
    assert_eq!(
        arr,
        &expected[..],
        "Partial sort mismatch.\n  input:  {:?}\n  got:    {:?}\n  expect: {:?}",
        original,
        arr,
        expected
    );

    // Subslice must be nondecreasing.
    assert!(
        arr[lo..hi].windows(2).all(|w| w[0] <= w[1]),
        "Subslice not in nondecreasing order after sort: {:?}",
        &arr[lo..hi]
    );

    // Idempotency on subslice.
    let mut again = arr.to_vec();
    sort(&mut again[lo..hi]);
    assert_eq!(again, expected, "Partial sort not idempotent");
}

fn pseudo_random_vec(len: usize, mut seed: u64, range: isize) -> Vec<isize> {
    fn xorshift64star(x: &mut u64) -> u64 {
        let mut z = *x;
        z ^= z >> 12;
        z ^= z << 25;
        z ^= z >> 27;
        *x = z;
        z.wrapping_mul(2685821657736338717)
    }
    let mut out = Vec::with_capacity(len);
    for _ in 0..len {
        let r = xorshift64star(&mut seed);
        let v = (r % ((2 * (range as u64)) + 1)) as i128 - (range as i128);
        out.push(v as isize);
    }
    out
}

// ---- Individual scenario tests ----

fn test_empty(sort: IntegerSortingAlgorithm) {
    let mut arr: [isize; 0] = [];
    sort(&mut arr);
    assert_eq!(arr, []);
}

fn test_singleton(sort: IntegerSortingAlgorithm) {
    let mut arr = [42isize];
    run_and_compare_full(sort, &mut arr);
}

fn test_two_elements(sort: IntegerSortingAlgorithm) {
    let mut a1 = [1isize, 2];
    let mut a2 = [2isize, 1];
    let mut a3 = [5isize, 5];

    run_and_compare_full(sort, &mut a1);
    run_and_compare_full(sort, &mut a2);
    run_and_compare_full(sort, &mut a3);
}

fn test_already_sorted(sort: IntegerSortingAlgorithm) {
    let mut arr: Vec<isize> = (0..128).collect();
    run_and_compare_full(sort, &mut arr);
}

fn test_reverse_order(sort: IntegerSortingAlgorithm) {
    let mut arr: Vec<isize> = (0..256).rev().collect();
    run_and_compare_full(sort, &mut arr);
}

fn test_all_same(sort: IntegerSortingAlgorithm) {
    let mut arr = vec![7isize; 257];
    run_and_compare_full(sort, &mut arr);
}

fn test_duplicates_dense(sort: IntegerSortingAlgorithm) {
    // Many repeated values (heavy duplicate distribution).
    let mut arr: Vec<isize> = (0..512).map(|i| (i % 7) as isize).collect();
    run_and_compare_full(sort, &mut arr);
}

fn test_negatives_and_mixed(sort: IntegerSortingAlgorithm) {
    let mut arr = vec![
        0, -1, 1, -2, 2, -3, 3, -4, 4, -5, 5, -6, 6, -7, 7, -8, 8, -9, 9,
    ];
    run_and_compare_full(sort, &mut arr);
}

fn test_extreme_values(sort: IntegerSortingAlgorithm) {
    let mut arr = vec![
        isize::MAX,
        0,
        isize::MIN,
        -1,
        1,
        isize::MIN,
        isize::MAX,
        2,
        -2,
        0,
    ];
    run_and_compare_full(sort, &mut arr);
}

fn test_nearly_sorted_one_swap(sort: IntegerSortingAlgorithm) {
    let mut arr: Vec<isize> = (0..200).collect();
    arr.swap(50, 150);
    run_and_compare_full(sort, &mut arr);
}

fn test_rotated_sorted(sort: IntegerSortingAlgorithm) {
    let base: Vec<isize> = (0..100).collect();
    for rot in [0usize, 1, 2, 3, 17, 50, 99] {
        let mut arr = base[rot..]
            .iter()
            .chain(base[..rot].iter())
            .cloned()
            .collect::<Vec<_>>();
        run_and_compare_full(sort, &mut arr);
    }
}

fn test_zigzag_pattern(sort: IntegerSortingAlgorithm) {
    // Alternating high/low to frustrate simplistic partitioners.
    let mut arr = Vec::with_capacity(201);
    for i in 0..100 {
        arr.push(10_000 - i as isize);
        arr.push(-10_000 + i as isize);
    }
    arr.push(0);
    run_and_compare_full(sort, &mut arr);
}

fn test_randomish_small(sort: IntegerSortingAlgorithm) {
    let mut arr = pseudo_random_vec(97, 0xDEADBEEFCAFEBABEu64, 2000);
    run_and_compare_full(sort, &mut arr);
}

fn test_randomish_medium(sort: IntegerSortingAlgorithm) {
    // Keep moderate so O(n^2) implementations still pass in reasonable time.
    let mut arr = pseudo_random_vec(1024, 0x1234_5678_9ABC_DEF0u64, 50_000);
    run_and_compare_full(sort, &mut arr);
}

fn test_partial_subslice_middle(sort: IntegerSortingAlgorithm) {
    let mut arr = pseudo_random_vec(200, 0xBEEFu64, 500);
    let orig = arr.clone();
    run_and_compare_partial(sort, &mut arr, 50, 150);

    // Double-check again with a different middle to exercise more boundaries.
    let mut arr2 = orig;
    run_and_compare_partial(sort, &mut arr2, 0, 200); // whole slice behaves the same as full
}

fn test_partial_edges(sort: IntegerSortingAlgorithm) {
    let mut arr = pseudo_random_vec(64, 0xA11CEu64, 100);
    // Sort prefix only
    run_and_compare_partial(sort, &mut arr, 0, 17);
    // Sort suffix only
    run_and_compare_partial(sort, &mut arr, 47, 64);
}

fn test_many_small_arrays(sort: IntegerSortingAlgorithm) {
    // Exhaustively test all arrays of length <= 5 over a tiny domain {-1,0,1}.
    // This is great for catching subtle boundary bugs.
    let domain = [-1isize, 0, 1];
    let mut buf = [0isize; 5];

    for len in 0..=5 {
        // Iterate all 3^len assignments.
        let total = 3usize.pow(len as u32);
        for mut code in 0..total {
            for i in 0..len {
                buf[i] = domain[code % 3];
                code /= 3;
            }
            let mut arr = buf[..len].to_vec();
            run_and_compare_full(sort, &mut arr);
        }
    }
}

// ---- Public entrypoint ----------------------------------------------------

pub fn test_sort(sort: IntegerSortingAlgorithm) {
    // Basic edges
    test_empty(sort);
    test_singleton(sort);
    test_two_elements(sort);

    // Structure and duplicates
    test_all_same(sort);
    test_already_sorted(sort);
    test_reverse_order(sort);
    test_duplicates_dense(sort);

    // Value ranges
    test_negatives_and_mixed(sort);
    test_extreme_values(sort);

    // Arrangement patterns
    test_nearly_sorted_one_swap(sort);
    test_rotated_sorted(sort);
    test_zigzag_pattern(sort);

    // Random-ish
    test_randomish_small(sort);
    test_randomish_medium(sort);

    // Sub-slice behavior
    test_partial_subslice_middle(sort);
    test_partial_edges(sort);

    // Many small arrays over a tiny domain
    test_many_small_arrays(sort);
}
