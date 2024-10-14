//{"name":"D. Yarik and Musical Notes","group":"Codeforces - Codeforces Round 909 (Div. 3)","url":"https://codeforces.com/contest/1899/problem/D","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n1\n2\n4\n3 1 3 2\n2\n1000 1000\n3\n1 1 1\n19\n2 4 1 6 2 8 5 4 2 10 5 10 8 7 4 3 2 6 10\n","output":"0\n2\n1\n3\n19\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DYarikAndMusicalNotes"}}}

use std::collections::BTreeMap;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let values = input.read_size_vec(n);

    let mut occurrences = BTreeMap::new();
    for v in values {
        *occurrences.entry(v).or_insert(0) += 1;
    }

    let mut answer = 0_u64;
    for (&value, &count) in &occurrences {
        if value == 1 || value == 2 {
            continue;
        }

        if count >= 2 {
            answer += count * (count - 1) / 2;
        }
    }

    let extra = occurrences.get(&1).unwrap_or(&0) + occurrences.get(&2).unwrap_or(&0);
    if extra >= 2 {
        answer += extra * (extra - 1) / 2;
    }

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
