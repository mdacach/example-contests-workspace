//{"name":"B. Hamon Odyssey","group":"Codeforces - Codeforces Round 882 (Div. 2)","url":"https://codeforces.com/contest/1847/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n3\n1 2 3\n5\n2 3 1 5 2\n4\n5 7 12 6\n","output":"1\n2\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BHamonOdyssey"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use std::ops::BitAnd;

type PreCalc = ();

const FULL_MASK: usize = (1 << 31) - 1;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let values = input.read_size_vec(n);

    let mut minimum = values[0];
    for i in 1..n {
        minimum = minimum.bitand(values[i]);
    }

    if minimum != 0 {
        out.print_line(1);
        return;
    }

    let mut groups = 1;
    let mut current_bitand = FULL_MASK;
    for i in 0..n {
        current_bitand = current_bitand.bitand(values[i]);
        if current_bitand == minimum {
            groups += 1;
            current_bitand = FULL_MASK;
        }
    }
    if current_bitand != minimum {
        groups -= 1;
    }
    out.print_line(groups);
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
