//{"name":"C. Factorials and Powers of Two","group":"Codeforces - Codeforces Round 774 (Div. 2)","url":"https://codeforces.com/problemset/problem/1646/C","interactive":false,"timeLimit":3000,"tests":[{"input":"4\n7\n11\n240\n17179869184\n","output":"2\n3\n4\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CFactorialsAndPowersOfTwo"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

const TOO_BIG: u64 = 1_000_000_000_005;

type PreCalc = Vec<u64>;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, factorials: &PreCalc) {
    let n = input.read_u64();

    let factorial_count = factorials.len();
    let mut answer = u64::MAX;
    for mask in 0..((1 << factorial_count) - 1) {
        let mut total = 0;
        let mut bits_used = 0;
        for bit in 0..15 {
            if (mask >> bit) & 1 == 1 {
                total += factorials[bit];
                bits_used += 1;
            }
        }

        if total <= n {
            let remaining = n - total;
            answer.minim(bits_used + remaining.count_ones() as u64);
        }
    }

    out.print_line(answer);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = {
        let mut factorials = vec![1];
        for i in 2..16 {
            factorials.push(factorials.last().unwrap() * (i as u64));
            if *factorials.last().unwrap() >= TOO_BIG {
                break;
            }
        }

        factorials
    };

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
