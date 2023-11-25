//{"name":"A. Game with Integers","group":"Codeforces - Codeforces Round 909 (Div. 3)","url":"https://codeforces.com/contest/1899/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n1\n3\n5\n100\n999\n1000\n","output":"First\nSecond\nFirst\nFirst\nSecond\nFirst\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AGameWithIntegers"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

// The first observation is that Vova can always "revert" Vanya's play.
// If Vanya adds 1, Vova subtracts 1. And the other way around.
//
// Let's see what happens with the first move. Because we only care about the divisibily by 3, we
// can only consider the original number modulo 3.
// If `n` % 3 == 0, after a move the remainder will either become 1 (if we add 1) or 2 (if we
// subtract 1). And because Vova can always revert this play, we get an infinite loop and Vova
// wins.
// If `n` % 3 == 1, we can subtract 1, the remainder becomes 0 and we win. Because Vanya always
// plays optimally, we don't care what happens otherwise - if we get here we win.
// Similarly, if `n` % 3 == 2, we can add 1, the remainder becomes 0 again, and we win again.
fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    if n % 3 != 0 {
        out.print_line("First");
    } else {
        out.print_line("Second");
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
