//{"name":"D. Jumping Through Segments","group":"Codeforces - Codeforces Round 913 (Div. 3)","url":"https://codeforces.com/contest/1907/problem/D","interactive":false,"timeLimit":5000,"tests":[{"input":"4\n5\n1 5\n3 4\n5 6\n8 10\n0 1\n3\n0 2\n0 1\n0 3\n3\n3 8\n10 18\n6 11\n4\n10 20\n0 5\n15 17\n2 2\n","output":"7\n0\n5\n13\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DJumpingThroughSegments"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let mut segments = vec![];
    for _ in 0..n {
        let segment = (input.read_size(), input.read_size());
        segments.push(segment);
    }

    let can_stay_put = segments.iter().all(|seg| seg.0 == 0);
    if can_stay_put {
        out.print_line(0);
        return;
    }

    let is_possible_with_step = |k: usize| {
        let mut possible_range = (0_usize, 0_usize);

        let current = 0;
        for i in 0..n {
            let (l, r) = segments[i];

            if i == n - 1 {
                // No need to worry.
            }
        }
        for &(l, r) in &segments {
            let is_disjoint = |(left, right)| r < left || l > right;
            let (current_left, current_right) = possible_range;
            possible_range = (
                std::cmp::max(current_left.saturating_sub(k), l),
                std::cmp::min(current_right.saturating_add(k), r),
            );

            if is_disjoint(possible_range) {
                return false;
            }
        }

        true
    };

    let mut left = 0; // Always impossible
    let mut right = 1_000_000_000; // Always possible
    while right - left > 1 {
        let middle = left + (right - left) / 2;
        if is_possible_with_step(middle) {
            right = middle;
        } else {
            left = middle;
        }
    }

    out.print_line(right);
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
