//{"name":"C. Place for a Selfie","group":"Codeforces - Codeforces Round 862 (Div. 2)","url":"https://codeforces.com/contest/1805/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n1 2\n1\n1 -1 2\n1 -1 3\n2 2\n1\n4\n1 2 1\n2 5 1\n1 1\n0\n1 0 0\n1 1\n100000000\n100000000 100000000 100000000\n2 3\n0\n2\n2 2 1\n1 -2 1\n1 -2 -1\n","output":"YES\n1\nYES\n1\n\nYES\n1\nYES\n4\n\nNO\n\nYES\n100000000\n\nYES\n0\nNO\nNO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CPlaceForASelfie"}}}

use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();

    let mut ks = input.read_long_vec(n);
    ks.sort_unstable();

    let mut answer = None;
    for _ in 0..m {
        let (a, b, c) = (input.read_long(), input.read_long(), input.read_long());
        let greater = ks.lower_bound(&b);
        let smaller = greater.saturating_sub(1);

        let right = 4 * a * c;
        if greater >= 0 && greater < n {
            let left = (b - ks[greater]) * (b - ks[greater]);
            if left < right {
                answer = Some(ks[greater]);
            }
        }

        if smaller >= 0 && smaller < n {
            let left = (b - ks[smaller]) * (b - ks[smaller]);
            if left < right {
                answer = Some(ks[smaller]);
            }
        }

        if let Some(answer) = answer {
            out.print_line("YES");
            out.print_line(answer);
        } else {
            out.print_line("NO");
        }
        answer = None;
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
