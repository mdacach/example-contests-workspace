//{"name":"D. Three Activities","group":"Codeforces - Codeforces Round 916 (Div. 3)","url":"https://codeforces.com/contest/1914/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n3\n1 10 1\n10 1 1\n1 1 10\n4\n30 20 10 1\n30 5 15 20\n30 25 10 10\n10\n5 19 12 3 18 18 6 17 10 13\n15 17 19 11 16 3 11 17 17 17\n1 17 18 10 15 8 17 3 13 12\n10\n17 5 4 18 12 4 11 2 16 16\n8 4 14 19 3 12 6 7 5 16\n3 4 8 11 10 8 10 2 20 3\n","output":"30\n75\n55\n56\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DThreeActivities"}}}

use std::collections::BTreeSet;
use std::fs::set_permissions;

use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

pub fn next_permutation<T>(arr: &mut [T]) -> bool
where
    T: std::cmp::Ord,
{
    use std::cmp::Ordering;

    // find 1st pair (x, y) from back which satisfies x < y
    let last_ascending = match arr.windows(2).rposition(|w| w[0] < w[1]) {
        Some(i) => i,
        None => {
            arr.reverse();
            return false;
        }
    };

    // In the remaining later segment, find the one which is just
    // larger that the index found above.
    // SAFETY: unwrap_err whill not panic since binary search will
    // will never succeed since we never return equal ordering
    let swap_with = arr[last_ascending + 1..]
        .binary_search_by(|n| match arr[last_ascending].cmp(n) {
            Ordering::Equal => Ordering::Greater,
            ord => ord,
        })
        .unwrap_err();
    arr.swap(last_ascending, last_ascending + swap_with);
    arr[last_ascending + 1..].reverse();
    true
}
fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_long_vec(n);
    let b = input.read_long_vec(n);
    let c = input.read_long_vec(n);

    let values = vec![a, b, c];
    let mut order = vec![0, 1, 2];

    let mut best_answer = 0;
    loop {
        let mut this_answer = 0;

        let mut used_indexes = vec![];
        for &current_vec in &order {
            let mut max = (0, 0);
            for (i, v) in values[current_vec].iter().enumerate() {
                if used_indexes.contains(&i) {
                    continue;
                }

                if *v >= max.0 {
                    max.0 = *v;
                    max.1 = i;
                }
            }

            this_answer += max.0;
            used_indexes.push(max.1);
        }

        best_answer.maxim(this_answer);

        if !next_permutation(&mut order) {
            break;
        }
    }

    out.print_line(best_answer);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 0usize..t {
                solve(&mut input, &mut output, i + 1, &pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
