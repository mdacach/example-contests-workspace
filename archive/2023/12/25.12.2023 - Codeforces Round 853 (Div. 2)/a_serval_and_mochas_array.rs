//{"name":"A. Serval and Mocha's Array","group":"Codeforces - Codeforces Round 853 (Div. 2)","url":"https://codeforces.com/contest/1789/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n2\n3 6\n3\n1 2 4\n3\n3 6 1\n3\n15 35 21\n4\n35 10 35 14\n5\n1261 227821 143 4171 1941\n","output":"No\nYes\nYes\nNo\nYes\nYes\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AServalAndMochasArray"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::numbers::gcd::gcd;

type PreCalc = ();

// The most important part of the resulting array is the first two numbers.
// Call them a and b.
// We must have that gcd(a, b) <= 2 (otherwise the array is not beautiful).
// So there is a _necessary_ condition: a pair of numbers such that their gcd is at most 2.
// We can also see that this condition is also _sufficient_.
//  Suppose you have such a pair. Because the gcd of the prefix can only _decrease_, all of the
//  gcds will also be at most 2, and thus at most the length of the prefixes.
//
// Because this condition is both necessary and sufficient, we need only check for that.
fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let values = input.read_size_vec(n);

    for i in 0..n {
        for j in 0..n {
            if gcd(values[i], values[j]) <= 2 {
                out.print_line("YES");
                return;
            }
        }
    }

    out.print_line("NO");
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
