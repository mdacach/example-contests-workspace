//{"name":"B. Chip and Ribbon","group":"Codeforces - Educational Codeforces Round 158 (Rated for Div. 2)","url":"https://codeforces.com/contest/1901/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n4\n1 2 2 1\n5\n1 0 1 0 1\n5\n5 4 3 2 1\n1\n12\n","output":"1\n2\n4\n11\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BChipAndRibbon"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let values = input.read_size_vec(n);

    let mut teleports = 0;
    let mut current_passes = 0;

    teleports += values[0];
    current_passes += values[0];
    for i in 1..n {
        if values[i] <= current_passes {
            current_passes.minim(values[i]);
        } else {
            teleports += values[i].abs_diff(current_passes);
            current_passes = values[i];
        }
    }
    out.print_line(teleports - 1);
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
