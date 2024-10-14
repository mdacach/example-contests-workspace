//{"name":"F. Alex's whims","group":"Codeforces - Codeforces Round 909 (Div. 3)","url":"https://codeforces.com/contest/1899/problem/F?locale=en","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n3 3\n2\n2\n2\n5 6\n4\n2\n3\n4\n3\n2\n4 9\n2\n3\n3\n2\n2\n2\n3\n2\n2\n","output":"1 2\n2 3\n-1 -1 -1\n-1 -1 -1\n-1 -1 -1\n1 2\n2 3\n3 4\n4 5\n-1 -1 -1\n4 3 2\n5 4 3\n4 2 5\n4 5 2\n5 3 4\n1 2\n2 3\n3 4\n4 3 2\n4 2 3\n-1 -1 -1\n4 3 2\n-1 -1 -1\n-1 -1 -1\n4 2 3\n4 3 2\n-1 -1 -1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FAlexsWhims"}}}

use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::collections::slice_ext::consecutive_iter::{self, ConsecutiveIter};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let (n, q) = (input.read_size(), input.read_size());
    let queries = input.read_size_vec(n);

    (1..=n)
        .collect_vec()
        .consecutive_iter()
        .for_each(|(a, b)| out.print_line(format!("{} {}", a, b)));

    out.print_line("");
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
