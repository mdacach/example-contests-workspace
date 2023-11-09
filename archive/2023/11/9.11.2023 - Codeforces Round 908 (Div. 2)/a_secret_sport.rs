//{"name":"A. Secret Sport","group":"Codeforces - Codeforces Round 908 (Div. 2)","url":"https://codeforces.com/contest/1894/problem/A","interactive":false,"timeLimit":3000,"tests":[{"input":"7\n5\nABBAA\n3\nBBB\n7\nBBAAABA\n20\nAAAAAAAABBBAABBBBBAB\n1\nA\n13\nAAAABABBABBAB\n7\nBBBAAAA\n","output":"A\nB\nA\nB\nA\nB\nA\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ASecretSport"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let s = input.read_string();
    out_line!(s.chars().last().unwrap());
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
