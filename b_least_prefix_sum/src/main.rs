//{"name":"B. Least Prefix Sum","group":"Codeforces - [1600 x4]","url":"https://codeforces.com/group/QsHQUUet4f/contest/489465/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n4 3\n-1 -2 -3 -4\n4 3\n1 2 3 4\n1 1\n1\n5 5\n-2 3 -5 1 -20\n5 2\n-2 3 -5 -5 -20\n10 4\n345875723 -48 384678321 -375635768 -35867853 -35863586 -358683842 -81725678 38576 -357865873\n","output":"1\n1\n0\n0\n3\n4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BLeastPrefixSum"}}}

use algo_lib::collections::slice_ext::reversed::ReversedSlice;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let (n, m) = (input.read_size(), input.read_size());
    let values = input.read_long_vec(n);

    let mut operations = 0;
    let (left, right) = values.split_at(m);
    let mut cumulative_sum = 0;
    for &x in right {
        if cumulative_sum + x < 0 {
            operations += 1;
            cumulative_sum -= x;
        } else {
            cumulative_sum += x;
        }
        assert!(cumulative_sum >= 0);
    }

    let mut left = left.to_owned();
    left.reverse();
    left.pop();
    cumulative_sum = 0;
    for x in left {
        if cumulative_sum + x > 0 {
            operations += 1;
            cumulative_sum -= x;
        } else {
            cumulative_sum += x;
        }
    }
    assert!(cumulative_sum <= 0);

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
