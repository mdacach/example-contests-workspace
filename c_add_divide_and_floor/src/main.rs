//{"name":"C. Add, Divide and Floor","group":"Codeforces - Educational Codeforces Round 158 (Rated for Div. 2)","url":"https://codeforces.com/contest/1901/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n1\n10\n2\n4 6\n6\n2 1 2 1 2 1\n2\n0 32\n","output":"0\n2\n2 5\n1\n1\n6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CAddDivideAndFloor"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let values = input.read_long_vec(n);

    let mut min = *values.iter().min().unwrap();
    let mut max = *values.iter().max().unwrap();

    let mut all_operations = vec![];
    let mut operations = 0;
    while max - min > 0 {
        let mid = (max + min) / 2;
        min = (min + mid) / 2;
        max = (max + mid) / 2;
        operations += 1;
        if operations <= n {
            all_operations.push(mid);
        }
    }

    out.print_line(operations);
    if operations <= n {
        out.print_line(all_operations);
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
