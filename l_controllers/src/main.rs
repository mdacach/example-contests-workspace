//{"name":"L. Controllers","group":"Codeforces - SWERC 2022-2023 - Online Mirror (Unrated, ICPC Rules, Teams Preferred)","url":"https://codeforces.com/problemset/problem/1776/L","interactive":false,"timeLimit":2000,"tests":[{"input":"8\n+-+---+-\n5\n2 1\n10 3\n7 9\n10 10\n5 3\n","output":"YES\nNO\nNO\nNO\nYES\n"},{"input":"6\n+-++--\n2\n9 7\n1 1\n","output":"YES\nYES\n"},{"input":"20\n+-----+--+--------+-\n2\n1000000000 99999997\n250000000 1000000000\n","output":"NO\nYES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"LControllers"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let signs = input.read_str();

    let mut P = 0_i64;
    let mut M = 0_i64;
    for s in signs {
        if s == b'+' {
            P += 1;
        } else if s == b'-' {
            M += 1;
        }
    }

    let q = input.read_size();
    for _ in 0..q {
        let (x, y) = (input.read_long(), input.read_long());
        if x == y {
            if P == M {
                out.print_line("YES");
            } else {
                out.print_line("NO");
            }
            continue;
        }
        // After a bunch of math.
        let numerator = x * (M - P);
        let denominator = x - y;
        if numerator % denominator != 0 {
            out.print_line("NO");
            continue;
        }

        let result = numerator / denominator;

        if (-P..=M).contains(&result) {
            out.print_line("YES");
        } else {
            out.print_line("NO");
        }
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
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
