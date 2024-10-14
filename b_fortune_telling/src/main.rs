//{"name":"B. Fortune Telling","group":"Codeforces - Codeforces Round 770 (Div. 2)","url":"https://codeforces.com/problemset/problem/1634/B","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n1 7 9\n2\n2 0 2\n1 3\n4 0 1\n1 2 3 4\n2 1000000000 3000000000\n1000000000 1000000000\n","output":"Alice\nAlice\nBob\nAlice\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BFortuneTelling"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let (n, x, y) = (input.read_size(), input.read_u64(), input.read_u64());
    let values = input.read_u64_vec(n);
    let sum: u64 = values.into_iter().sum();

    match (x % 2 == 0, y % 2 == 0) {
        (true, true) | (false, false) => {
            if sum % 2 == 0 {
                out.print_line("Alice");
            } else {
                out.print_line("Bob");
            }
        }
        _ => {
            if sum % 2 == 0 {
                out.print_line("Bob");
            } else {
                out.print_line("Alice");
            }
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
