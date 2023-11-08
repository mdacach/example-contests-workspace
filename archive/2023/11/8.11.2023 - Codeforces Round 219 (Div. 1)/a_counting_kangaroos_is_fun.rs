//{"name":"A. Counting Kangaroos is Fun","group":"Codeforces - Codeforces Round 219 (Div. 1)","url":"https://codeforces.com/problemset/problem/372/A","interactive":false,"timeLimit":1000,"tests":[{"input":"8\n2\n5\n7\n6\n9\n8\n4\n2\n","output":"5\n"},{"input":"8\n9\n1\n6\n2\n6\n5\n8\n3\n","output":"5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ACountingKangaroosIsFun"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let mut sizes = input.read_int_vec(n);
    sizes.sort_unstable();

    let is_possible = |in_pocket: usize| {
        if in_pocket * 2 > n {
            return false;
        }

        let small = &sizes[..in_pocket];
        let big = &sizes[n - in_pocket..];
        small
            .into_iter()
            .zip(big.into_iter())
            .all(|(&a, &b)| a * 2 <= b)
    };

    let mut left = 0;
    let mut right = n;
    while right - left > 1 {
        let middle = left + (right - left) / 2;
        if is_possible(middle) {
            left = middle;
        } else {
            right = middle;
        }
    }

    let visible = n - left;
    out_line!(visible);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
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
