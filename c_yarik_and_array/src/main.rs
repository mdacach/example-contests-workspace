//{"name":"C. Yarik and Array","group":"Codeforces - Codeforces Round 909 (Div. 3)","url":"https://codeforces.com/contest/1899/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"7\n5\n1 2 3 4 5\n4\n9 9 8 8\n6\n-1 4 -1 0 5 -4\n4\n-1 2 4 -3\n1\n-1000\n3\n101 -99 101\n20\n-10 5 -8 10 6 -10 7 9 -2 -6 7 2 -4 6 -1 7 -6 -7 4 1\n","output":"15\n17\n8\n4\n-1000\n101\n10\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CYarikAndArray"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let values = input.read_long_vec(n);

    let mut possible = vec![];
    let mut last = values[0].abs() % 2;
    let mut current = vec![values[0]];
    for i in 1..n {
        if values[i].abs() % 2 == last {
            possible.push(current);
            current = vec![values[i]];
        } else {
            current.push(values[i]);
            last = last ^ 1;
        }
    }
    if !current.is_empty() {
        possible.push(current);
    }

    let kadane = |values: Vec<i64>| {
        let mut best = values[0];
        let mut best_until_here = values[0];
        for v in values.into_iter().skip(1) {
            if best_until_here < 0 {
                best_until_here = v;
            } else {
                best_until_here += v;
            }
            best.maxim(best_until_here);
        }
        best
    };

    let answer: i64 = possible.into_iter().map(|v| kadane(v)).max().unwrap();
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
