use crate::collections::{min_max::MinimMaxim, slice_ext::bounds::Bounds};

pub fn longest_increasing_subsequence(values: &[i64]) -> usize {
    let n = values.len();
    debug_assert!(n > 0);

    let mut min_end_with_length = vec![i64::MAX; n + 1];
    min_end_with_length[0] = 0;
    for v in values {
        let pos = min_end_with_length.lower_bound(v);
        if pos <= n {
            min_end_with_length[pos].minim(*v);
        }
    }

    if let Some(first_not_possible) = min_end_with_length.into_iter().position(|v| v == i64::MAX) {
        first_not_possible - 1
    } else {
        n
    }
}

#[test]
fn easy_sequences() {
    let values = vec![1, 2, 3, 4, 5];
    assert_eq!(longest_increasing_subsequence(&values), 5);

    let values = vec![1];
    assert_eq!(longest_increasing_subsequence(&values), 1);
}

#[test]
fn no_sequence() {
    let values = vec![5, 3, 2, 2, 1];
    assert_eq!(longest_increasing_subsequence(&values), 1);
}

#[test]
fn tricky() {
    let values = vec![1, 7, 2, 8, 9, 3, 10, 5, 12, 5, 2, 3, 4, 5, 6, 7];
    assert_eq!(longest_increasing_subsequence(&values), 7);
}
