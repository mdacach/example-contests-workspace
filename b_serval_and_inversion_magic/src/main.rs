//{"name":"B. Serval and Inversion Magic","group":"Codeforces - Codeforces Round 853 (Div. 2)","url":"https://codeforces.com/contest/1789/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n4\n1001\n5\n10010\n7\n0111011\n","output":"Yes\nYes\nNo\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BServalAndInversionMagic"}}}

use algo_lib::collections::slice_ext::consecutive_iter::{self, ConsecutiveIter};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let s = input.read_str();

    let mut must_change = vec![];
    for i in 0..n / 2 {
        if s[i] != s[n - i - 1] {
            must_change.push(i);
        }
    }

    let good = must_change.consecutive_iter().all(|(&a, &b)| b == a + 1);
    out.print_line(if good { "YES" } else { "NO " });
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
