//{"name":"E. Block Sequence","group":"Codeforces - Codeforces Round 903 (Div. 3)","url":"https://codeforces.com/problemset/problem/1881/E","interactive":false,"timeLimit":2000,"tests":[{"input":"7\n7\n3 3 4 5 2 6 1\n4\n5 6 3 2\n6\n3 4 1 6 7 7\n3\n1 4 3\n5\n1 2 3 4 5\n5\n1 2 3 1 2\n5\n4 5 5 1 5\n","output":"0\n4\n1\n1\n2\n1\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EBlockSequence"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve_recursively(
    index: usize,
    values: &Vec<usize>,
    already_computed: &mut Vec<Option<usize>>,
) -> usize {
    if index == values.len() {
        // We successfully exhausted the whole array.
        return 0;
    }
    if index > values.len() {
        // This must be the result of a bad transition.
        return usize::MAX;
    }

    if let Some(v) = already_computed[index] {
        return v;
    }

    let skip_current_value =
        1_usize.saturating_add(solve_recursively(index + 1, values, already_computed));
    let pick_as_leader_of_block =
        solve_recursively(index + 1 + values[index], values, already_computed);

    let solution = std::cmp::min(skip_current_value, pick_as_leader_of_block);
    already_computed[index] = Some(solution);
    solution
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let values = input.read_size_vec(n);
    let mut empty_vec = vec![None; n];

    let answer = solve_recursively(0, &values, &mut empty_vec);
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
