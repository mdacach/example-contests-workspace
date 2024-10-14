//{"name":"C. Non-coprime Split","group":"Codeforces - Codeforces Round 895 (Div. 3)","url":"https://codeforces.com/contest/1872/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"11\n11 15\n1 3\n18 19\n41 43\n777 777\n8000000 10000000\n2000 2023\n1791791 1791791\n1 4\n2 3\n9840769 9840769\n","output":"6 9\n-1\n14 4\n36 6\n111 666\n4000000 5000000\n2009 7\n-1\n2 2\n-1\n6274 9834495\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CNonCoprimeSplit"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn factor(mut number: i32) -> Vec<i32> {
    let mut divisors = vec![1, number];
    let mut d = 2;
    while d * d <= number {
        if number % d == 0 {
            divisors.push(d);
            divisors.push(number / d);
        }
        d += 1;
    }
    divisors.sort();
    divisors
}

fn solve(input: &mut Input, _test_case: usize) {
    let (l, r) = (input.read_int(), input.read_int());
    if (l == r) {
        let divisors = factor(l);
        if divisors.len() == 2 {
            assert!(divisors[0] == 1);
            out_line!(-1);
        } else {
            let d = divisors[1];
            let other = l / d;
            out_line!(d * 1, d * (other - 1));
        }
    } else {
        if r % 2 == 0 {
            if r == 2 {
                out_line!(-1);
            } else {
                out_line!(2, r - 2);
            }
        } else {
            if (r - 1) == 2 {
                out_line!(-1);
            } else {
                out_line!(2, (r - 1) - 2);
            }
        }
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
    }
    output().flush();
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
