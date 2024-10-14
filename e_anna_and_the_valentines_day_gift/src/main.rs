//{"name":"E. Anna and the Valentine's Day Gift","group":"Codeforces - Codeforces Round 925 (Div. 3)","url":"https://codeforces.com/problemset/problem/1931/E","interactive":false,"timeLimit":2000,"tests":[{"input":"9\n2 2\n14 2\n3 5\n9 56 1\n4 10\n1 2007 800 1580\n4 5\n5000 123 30 4\n10 10\n6 4 6 2 3 1 10 9 10 7\n1 1\n6\n1 1\n10\n8 9\n1 2 9 10 10 2 10 2\n4 5\n10 10 10 10\n","output":"Sasha\nAnna\nAnna\nSasha\nSasha\nAnna\nAnna\nAnna\nSasha\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EAnnaAndTheValentinesDayGift"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn num_trailing_zeros(mut number: i64) -> usize {
    let mut zeros = 0;
    while number != 0 {
        let last_digit = number % 10;
        if last_digit != 0 {
            break;
        }
        zeros += 1;
        number /= 10;
    }
    zeros
}

fn num_digits(mut number: i64) -> usize {
    let mut digits = 0;
    while number != 0 {
        digits += 1;
        number /= 10;
    }
    digits
}
fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let (n, m) = (input.read_size(), input.read_size());

    let mut numbers = input.read_long_vec(n);
    numbers.sort_by_key(|a| num_trailing_zeros(*a));

    let mut initial_answer: usize = numbers.iter().map(|a| num_digits(*a)).sum();

    loop {
        if numbers.len() < 1 {
            break;
        }

        // Anna always reverts the number with most trailing zeros, as those will get deleted.
        let first = numbers.pop().unwrap();
        let zeros = num_trailing_zeros(first);
        initial_answer -= zeros;

        if numbers.len() < 2 {
            break;
        }

        // Sasha concatenates also the number with most trailing zeros, to avoid Anna deleting them.
        numbers.pop().unwrap();
    }

    if initial_answer > m {
        out.print_line("Sasha");
    } else {
        out.print_line("Anna");
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
