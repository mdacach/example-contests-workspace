//{"name":"B. XOR Palindromes","group":"Codeforces - Codeforces Round 897 (Div. 2)","url":"https://codeforces.com/contest/1867/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n6\n101011\n5\n00000\n9\n100100011\n3\n100\n1\n1\n","output":"0010100\n111111\n0011111100\n0110\n11\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BXORPalindromes"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let s = input.read_str();

    let mut need_to_change = 0;
    for i in 0..(n / 2) {
        if s[i] != s[n - 1 - i] {
            need_to_change += 1;
        }
    }

    let mut answer = String::new();
    for i in 0..=n {
        if i < need_to_change {
            answer.push('0');
        } else {
            let need_to_change_extra = i - need_to_change;
            let is_ok_extra = (n - 2 * need_to_change);

            if need_to_change_extra <= is_ok_extra {
                if need_to_change_extra % 2 == 0 {
                    answer.push('1');
                } else {
                    if n % 2 == 1 {
                        answer.push('1');
                    } else {
                        answer.push('0');
                    }
                }
            } else {
                answer.push('0');
            }
        }
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
