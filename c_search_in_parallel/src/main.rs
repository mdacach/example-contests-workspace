//{"name":"C. Search in Parallel","group":"Codeforces - [1200, 1300, 1500, 1600]","url":"https://codeforces.com/group/QsHQUUet4f/contest/489749/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n7 3 1\n8 6 4 4 4 1 7\n5 1 10\n1 1 1 1 1\n8 1 1\n4 5 6 8 1 7 3 2\n","output":"2 5 6\n5 1 7 2 4 3\n5 4 3 5 2 1\n0\n4 4 2 7 5\n4 6 3 1 8\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CSearchInParallel"}}}

use std::boxed;

use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let (n, s1, s2) = (input.read_size(), input.read_long(), input.read_long());
    let box_occurrences = input.read_long_vec(n);
    let mut boxes = box_occurrences.into_iter().enumerate().collect_vec();
    boxes.sort_by(|a, b| b.1.cmp(&a.1));

    let mut first_robot = vec![];
    let mut second_robot = vec![];

    for (id, occurrences) in boxes {
        let in_first = (first_robot.len() as i64 + 1) * s1 * occurrences;
        let in_second = (second_robot.len() as i64 + 1) * s2 * occurrences;

        if in_first <= in_second {
            first_robot.push(id + 1);
        } else {
            second_robot.push(id + 1);
        }
    }

    out.print(format!("{} ", first_robot.len()));
    out.print_line(first_robot);

    out.print(format!("{} ", second_robot.len()));
    out.print_line(second_robot);
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
