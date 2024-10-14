//{"name":"B. Iva & Pav","group":"Codeforces - [1300, 1400, 1500, 1600]","url":"https://codeforces.com/group/QsHQUUet4f/contest/488797/problem/B","interactive":false,"timeLimit":5000,"tests":[{"input":"3\n5\n15 14 17 42 34\n3\n1 7\n2 15\n4 5\n5\n7 5 3 1 7\n4\n1 7\n5 7\n2 3\n2 2\n7\n19 20 15 12 21 7 11\n4\n1 15\n4 4\n7 12\n5 7\n","output":"2 -1 5\n1 5 2 2\n2 6 -1 5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BIvaPav"}}}

use algo_lib::collections::iter_ext::collect::{self, IterCollect};
use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::numbers::num_utils::PartialSums;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_u64_vec(n);
    let q = input.read_size();

    // let mut cumulative_sums = vec![vec![]; 32];
    // for b in 0..32 {
    //     let for_this_bit = a.iter().map(|x| (x >> b) & 1).collect_vec();
    //     let sums = for_this_bit.partial_sums();
    //     cumulative_sums[b] = sums;
    // }

    #[derive(Clone, Copy)]
    struct Node {
        and: u64,
    };

    impl SegmentTreeNode for Node {
        fn new(left: usize, right: usize) -> Self {
            Self { and: u64::MAX - 1 }
        }

        fn join(
            &mut self,
            left_val: &Self,
            right_val: &Self,
            left: usize,
            mid: usize,
            right: usize,
        ) {
            self.and = left_val.and & right_val.and;
        }

        fn accumulate(&mut self, value: &Self, left: usize, right: usize) {}

        fn reset_delta(&mut self, left: usize, right: usize) {}
    }

    let mut and_segment_tree = SegmentTree::from_generator(n, |at| Node { and: a[at] as u64 });

    for _ in 0..q {
        let (l, k) = (input.read_size(), input.read_u64());
        if a[l - 1] < k {
            out.print(format!("{} ", -1));
            continue;
        }

        let mut is_good = |r: usize| {
            if r > n {
                return false;
            }

            let res = and_segment_tree.query(l - 1..r);
            res.and >= k

            // let mut res = 0;
            // for b in 0..32 {
            //     let is_on = cumulative_sums[b][r] - cumulative_sums[b][l - 1] == r - l + 1;
            //     if is_on {
            //         res += (1 << b);
            //     }
            // }
            //
            // res >= k
        };

        let mut left = l;
        let mut right = n + 1;
        while right - left > 1 {
            let middle = left + (right - left) / 2;
            if is_good(middle) {
                left = middle;
            } else {
                right = middle;
            }
        }
        out.print(format!("{} ", left));
    }
    out.print_line("");
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
