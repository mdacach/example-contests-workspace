//{"name":"A. Line Trip","group":"Codeforces - Educational Codeforces Round 158 (Rated for Div. 2)","url":"https://codeforces.com/contest/1901/problem/A","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n3 7\n1 2 5\n3 6\n1 2 5\n1 10\n7\n","output":"4\n3\n7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ALineTrip"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

// Because refueling costs nothing, we should always refuel every time we reach a gas station.
// This means that we only need to have enough fuel to go from a gas station to the next one.
// There are two special cases:
// 1. At the start, you need to have enough fuel to reach the first gas station from point zero.
// 2. At the end of the route, you need to have enough fuel to go from the last gas station, to
//    point x, and back.
//
// Code is simpler because they guarantee there are no gas stations at point zero nor at
// point x.
fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let (n, x) = (input.read_size(), input.read_size());
    let stations = input.read_size_vec(n);

    let mut maximum_dist = 0;
    let mut last: usize = 0;
    for &p in &stations {
        maximum_dist.maxim(last.abs_diff(p));
        last = p;
    }

    maximum_dist.maxim(2 * (last.abs_diff(x)));
    out.print_line(maximum_dist);
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
