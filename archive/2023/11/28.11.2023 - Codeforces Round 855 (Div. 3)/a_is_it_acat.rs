//{"name":"A. Is It a Cat?","group":"Codeforces - Codeforces Round 855 (Div. 3)","url":"https://codeforces.com/contest/1800/problem/A","interactive":false,"timeLimit":2000,"tests":[{"input":"7\n4\nmeOw\n14\nmMmeoOoWWWwwwW\n3\nmew\n7\nMmeEeUw\n4\nMEOW\n6\nMmyaVW\n5\nmeowA\n","output":"YES\nYES\nNO\nNO\nYES\nNO\nNO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AIsItACat"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::StrReader;

type PreCalc = ();

// Actually coded this wrong lol. That's what you get for not running local tests.
fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();

    let s = input.read_str();

    let mut last = 'z';
    for c in s {
        let c = c as char;
        match c.to_ascii_lowercase() {
            'm' => {
                if matches!(last, 'z' | 'm') {
                    last = 'm';
                } else {
                    out.print_line("NO");
                    return;
                }
            }
            'e' => {
                if matches!(last, 'm' | 'e') {
                    last = 'e';
                } else {
                    out.print_line("NO");
                    return;
                }
            }
            'o' => {
                if matches!(last, 'e' | 'o') {
                    last = 'o';
                } else {
                    out.print_line("NO");
                    return;
                }
            }
            'w' => {
                if matches!(last, 'o' | 'w') {
                    last = 'w';
                } else {
                    out.print_line("NO");
                    return;
                }
            }
            _ => {
                out.print_line("NO");
                return;
            }
        }
    }
    if last == 'w' {
        out.print_line("YES");
    } else {
        out.print_line("NO");
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
