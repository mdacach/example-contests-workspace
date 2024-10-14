//{"name":"C. Vika and Price Tags","group":"Codeforces - Codeforces Round 885 (Div. 2)","url":"https://codeforces.com/contest/1848/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"9\n4\n0 0 0 0\n1 2 3 4\n3\n1 2 3\n1 2 3\n2\n1 2\n2 1\n6\n100 23 53 11 56 32\n1245 31 12 6 6 6\n7\n1 2 3 4 5 6 7\n7 6 5 4 3 2 1\n3\n4 0 2\n4 0 2\n3\n2 5 2\n1 3 4\n2\n6 1\n4 2\n2\n0 0\n0 3\n","output":"YES\nYES\nNO\nNO\nYES\nYES\nNO\nYES\nYES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CVikaAndPriceTags"}}}

use std::collections::BTreeSet;
use std::mem::swap;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_long_vec(n);
    let b = input.read_long_vec(n);

    let mut remainders = BTreeSet::new();
    for i in 0..n {
        let (mut a, mut b) = (a[i], b[i]);
        if a == 0 && b == 0 {
            continue;
        }

        let mut count = 0;
        while b != 0 {
            if a < b {
                count += 1;
                let tmp = b;
                b = b - a;
                a = tmp;
            }

            // a = k*b + r
            // Eventually, a - k*b = r shows up.
            // If k is even, it will show up as (r, b)
            // if k is odd, it will show up as (b, r)
            let k = a / b;
            let r = a % b;

            if k % 2 == 0 {
                count += k + k / 2;
                a = r;
                if r == 0 {
                    count -= 1;
                    break;
                }
            } else {
                count += k + k / 2;
                a = b;
                b = r;
            }
        }

        remainders.insert(count % 3);
    }

    if remainders.len() == 1 {
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
