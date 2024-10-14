//{"name":"B. Hossam and Friends","group":"Codeforces - Codeforces Round 837 (Div. 2)","url":"https://codeforces.com/problemset/problem/1771/B","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n3 2\n1 3\n2 3\n4 2\n1 2\n2 3\n","output":"4\n5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BHossamAndFriends"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    #[derive(Clone, Copy)]
    struct Node {
        min: usize,
    };

    impl SegmentTreeNode for Node {
        fn new(left: usize, right: usize) -> Self {
            Self { min: usize::MAX }
        }

        fn join(
            &mut self,
            left_val: &Self,
            right_val: &Self,
            left: usize,
            mid: usize,
            right: usize,
        ) {
            self.min = std::cmp::min(left_val.min, right_val.min);
        }

        fn accumulate(&mut self, value: &Self, left: usize, right: usize) {}

        fn reset_delta(&mut self, left: usize, right: usize) {}
    }

    let (n, m) = (input.read_size(), input.read_size());
    let mut minimum_related = vec![n; n];
    for _ in 0..m {
        let (a, b) = (input.read_size() - 1, input.read_size() - 1);
        let min = std::cmp::min(a, b);
        let other = std::cmp::max(a, b);
        minimum_related[min].minim(other);
    }

    let mut min_segment_tree = SegmentTree::from_generator(n, |at| Node {
        min: minimum_related[at],
    });

    let mut good_subsegments = 0;
    for starting_point in 0..n {
        let mut is_ok = |mid| min_segment_tree.query(starting_point..=mid).min > mid;

        let mut left = starting_point;
        let mut right = n;
        while right - left > 1 {
            let mid = left + (right - left) / 2;
            if is_ok(mid) {
                left = mid;
            } else {
                right = mid;
            }
        }

        let answer = left;

        good_subsegments += answer - starting_point + 1;
    }

    out.print_line(good_subsegments);
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
