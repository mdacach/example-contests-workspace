//{"name":"D. Neutral Tonality","group":"Codeforces - Codeforces Round 908 (Div. 2)","url":"https://codeforces.com/contest/1894/problem/D","interactive":false,"timeLimit":3000,"tests":[{"input":"7\n2 1\n6 4\n5\n5 5\n1 7 2 4 5\n5 4 1 2 7\n1 9\n7\n1 2 3 4 5 6 7 8 9\n3 2\n1 3 5\n2 4\n10 5\n1 9 2 3 8 1 4 7 2 9\n7 8 5 4 6\n2 1\n2 2\n1\n6 1\n1 1 1 1 1 1\n777\n","output":"6 5 4\n1 1 7 7 2 2 4 4 5 5\n9 8 7 7 6 5 4 3 2 1\n1 3 5 2 4\n1 9 2 3 8 8 1 4 4 7 7 2 9 6 5\n2 2 1\n777 1 1 1 1 1 1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DNeutralTonality"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::Bounds;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn longest_increasing_subsequence(values: &[usize]) -> usize {
    let n = values.len();
    let mut min_end_with_length = vec![usize::MAX; n + 1];
    min_end_with_length[0] = 0;
    for x in values {
        let pos = min_end_with_length.lower_bound(x);
        if pos <= n {
            min_end_with_length[pos].minim(*x);
        }
    }

    if let Some(first_none) = min_end_with_length
        .into_iter()
        .position(|v| v == usize::MAX)
    {
        first_none - 1
    } else {
        n
    }
}

#[test]
fn test_lis() {
    let values = vec![5, 1, 2, 3, 5, 10, 8, 9, 10, 11, 14, 3000, 3];
    let res = longest_increasing_subsequence(&values);
    assert_eq!(res, 10);
}

fn solve(input: &mut Input, _test_case: usize) {
    let (n, m) = (input.read_size(), input.read_size());
    let a = input.read_size_vec(n);
    let b = input.read_size_vec(m);

    // It's not true that you always add a prefix and a suffix, lol
    // Just look at test case 1.
    // let original_lis = longest_increasing_subsequence(&a);
    // let same_lis_adding = |v: usize| {
    //     let mut new_array = vec![v];
    //     new_array.extend_from_slice(&a);
    //
    //     let new_lis = longest_increasing_subsequence(&new_array);
    //     new_lis == original_lis
    // };
    // let mut left = 0;
    // let mut right = usize::MAX;
    // while right - left > 1 {
    //     let middle = left + (right - left) / 2;
    //     if same_lis_adding(middle) {
    //         right = middle;
    //     } else {
    //         left = middle;
    //     }
    // }
    //
    // let cutoff = right;
    // let mut sorted_b = b;
    // sorted_b.sort_unstable_by(|a, b| b.cmp(&a));
    //
    // let mut answer = vec![];
    // let cutoff_point = sorted_b.iter().position(|v| *v < cutoff).unwrap_or(m);
    // let (big, small) = sorted_b.split_at(cutoff_point);
    //
    // answer.extend_from_slice(big);
    // answer.extend_from_slice(&a);
    // answer.extend_from_slice(small);
    //
    // out_line!(answer);
    //
    // let new_lis = longest_increasing_subsequence(&answer);
    // debug_assert!(new_lis == original_lis || new_lis == original_lis + 1);
    let mut sorted_b = b;
    sorted_b.sort_unstable_by(|a, b| b.cmp(&a));

    let mut answer = vec![];
    let mut index_b = 0;
    for value in a {
        while index_b < m && sorted_b[index_b] >= value {
            answer.push(sorted_b[index_b]);
            index_b += 1;
        }
        answer.push(value);
    }
    while index_b < m {
        answer.push(sorted_b[index_b]);
        index_b += 1;
    }
    out_line!(answer);
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
    }
    output().flush();
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
