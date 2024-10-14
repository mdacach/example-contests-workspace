//{"name":"C. Card Game","group":"Codeforces - [1300, 1400, 1500, 1600]","url":"https://codeforces.com/group/QsHQUUet4f/contest/488797/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n4\n-4 1 -3 5\n4\n1 -2 3 -4\n3\n-1 3 -5\n1\n-1\n","output":"5\n4\n2\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CCardGame"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let values = input.read_long_vec(n);

    let all_positive = values.iter().filter(|&&x| x >= 0).sum::<i64>();

    if values[0] >= 0 {
        // Can do all positive
        out.print_line(all_positive);
    } else {
        let first_positive = values.iter().position(|p| *p >= 0);
        let Some(first_positive) = first_positive else {
            out.print_line(0);
            return;
        };

        let pos = first_positive + 1;
        if pos % 2 == 1 {
            // Can do all positive
            out.print_line(all_positive);
        } else {
            // Will need to remove a negative or this guy
            // Can remove this guy and make all others good
            let mut answer = all_positive - values[pos - 1];
            // Or can remove a negative before
            // If there's an even negative, better
            if pos >= 3 {
                answer.maxim(all_positive);
            } else {
                assert!(pos == 2);
                if values[0].abs() < values[1] {
                    answer.maxim(all_positive - values[1] + values[1] - values[0].abs());
                } else {
                    answer.maxim(all_positive - values[1]);
                }
            }
            out.print_line(answer);
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
