//{"name":"B. The String Has a Target","group":"Codeforces - Codeforces Round 862 (Div. 2)","url":"https://codeforces.com/contest/1805/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n3\ncba\n4\nacac\n5\nabbcb\n4\naaba\n","output":"acb\naacc\nabbcb\naaab\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BTheStringHasATarget"}}}

use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let s = input.read_str();

    let minimum_char = s.iter().min().unwrap();
    let last_index = s.iter().rposition(|v| v == minimum_char).unwrap();

    let mut answer = s.iter().map(|c| c as char).collect::<String>();
    answer.remove(last_index);

    out.print(minimum_char as char);
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
