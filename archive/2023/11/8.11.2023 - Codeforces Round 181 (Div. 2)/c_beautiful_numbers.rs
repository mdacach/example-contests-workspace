//{"name":"C. Beautiful Numbers","group":"Codeforces - Codeforces Round 181 (Div. 2)","url":"https://codeforces.com/problemset/problem/300/C","interactive":false,"timeLimit":2000,"tests":[{"input":"1 3 3\n","output":"1\n"},{"input":"2 3 10\n","output":"165\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CBeautifulNumbers"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::modular_int::{Combinatorics, ModularInt};
use algo_lib::{out, out_line};

const MOD: i64 = 1_000_000_007;

fn solve(input: &mut Input, _test_case: usize) {
    let (a, b) = (input.read_size(), input.read_size());
    let n = input.read_size();

    let is_good = |mut number: usize| {
        let mut digits = vec![];
        while number > 0 {
            digits.push(number % 10);
            number /= 10;
        }
        digits.into_iter().all(|d| d == a || d == b)
    };

    let combinatorics = Combinatorics::<MOD>::new_with_range(n);

    let mut answer = ModularInt::<MOD>::from_small(0);
    for number_of_as in 0..=n {
        let number_of_bs = n - number_of_as;
        let sum_from_a = number_of_as * a;
        let sum_from_b = number_of_bs * b;
        let total_sum = sum_from_a + sum_from_b;

        if !is_good(total_sum) {
            continue;
        }

        let ways = combinatorics.combinations((n as i64).into(), (number_of_as as i64).into());
        answer = answer + ways;
    }

    out_line!(answer.value);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
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
