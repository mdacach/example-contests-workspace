//{"name":"C. Salyg1n and the MEX Game","group":"Codeforces - Codeforces Round 897 (Div. 2)","url":"https://codeforces.com/contest/1867/problem/C","interactive":true,"timeLimit":3000,"tests":[{"input":"3\n5\n1 2 3 5 7\n\n7\n\n5\n\n-1\n\n3\n0 1 2\n\n0\n\n-1\n\n3\n5 7 57\n\n-1\n","output":"8\n\n57\n\n0\n\n3\n\n0\n\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CSalyg1nAndTheMEXGame"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let starting_set = input.read_size_vec(n);

    let mut sorted = starting_set.clone();
    sorted.sort_unstable();
    let mut mex = 0;
    let mut i = 0;
    while i < n && sorted[i] == mex {
        mex += 1;
        i += 1;
    }

    out.print_line(mex);
    out.flush();

    loop {
        let removed = input.read_int();
        if removed == -1 {
            out.print_line("");
            return;
        } else if removed == -2 {
            out.print_line("");
            return;
        }

        out.print_line(removed);
        out.flush();
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
