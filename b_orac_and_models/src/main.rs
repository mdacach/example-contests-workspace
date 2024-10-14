//{"name":"B. Orac and Models","group":"Codeforces - Codeforces Round 641 (Div. 2)","url":"https://codeforces.com/problemset/problem/1350/B","interactive":false,"timeLimit":3000,"tests":[{"input":"4\n4\n5 3 4 6\n7\n1 4 2 3 6 4 9\n5\n5 4 3 2 1\n1\n9\n","output":"2\n3\n1\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BOracAndModels"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let values = input.read_u64_vec(n);

    let mut best_here = vec![1; n];
    for starting_index in 0..n {
        for multiple_index in (starting_index..n).step_by(starting_index + 1).skip(1) {
            if values[multiple_index] > values[starting_index] {
                let here = best_here[starting_index];
                best_here[multiple_index].maxim(here + 1);
            }
        }
    }

    let answer = best_here.iter().max().unwrap();
    out.print_line(answer);
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
