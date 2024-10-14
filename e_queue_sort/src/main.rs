//{"name":"E. Queue Sort","group":"Codeforces - Codeforces Round 909 (Div. 3)","url":"https://codeforces.com/contest/1899/problem/E","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n5\n6 4 1 2 5\n7\n4 5 3 7 8 6 2\n6\n4 3 1 2 6 4\n4\n5 2 4 2\n3\n2 2 3\n","output":"2\n6\n-1\n-1\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EQueueSort"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let values = input.read_size_vec(n);

    let minimum = *values.iter().min().unwrap();
    let position = values.iter().position(|v| *v == minimum).unwrap();
    let mut operations = position;

    let mut last = minimum;
    for i in (position + 1)..n {
        if values[i] < last {
            out.print_line(-1);
            return;
        }
        last = values[i];
    }

    out.print_line(operations);
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
