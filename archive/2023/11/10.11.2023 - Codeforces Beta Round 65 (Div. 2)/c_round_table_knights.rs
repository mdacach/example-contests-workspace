//{"name":"C. Round Table Knights","group":"Codeforces - Codeforces Beta Round 65 (Div. 2)","url":"https://codeforces.com/problemset/problem/71/C","interactive":false,"timeLimit":500,"tests":[{"input":"3\n1 1 1\n","output":"YES\n"},{"input":"6\n1 0 1 1 1 0\n","output":"YES\n"},{"input":"6\n1 0 0 1 0 1\n","output":"NO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CRoundTableKnights"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::numbers::primes::factorize::Factorize;

type PreCalc = ();

// As the polygon must be regular, all of the sides must be of the same length.
// Because the sides must sum up to N, they must be some divisor of N.
//
// There's not many divisors of N (we can use N^1/3 as an upperbound).
// So if we can solve quickly enough for one given divisor, we are OK.
//
// Suppose that you know that each side must be of size K. Then, you must have
// knight[0] = knight[k] = knight[2k] ... = good
// But you can also start at other places,
// knight[1] = knight[k + 1] = knight[2k + 1] = good
// ...
// By the cyclic nature of the circle, when you arrive at knight[k] as a starting point, it is
// exactly the same thing as starting at knight[0].
//
// Thus that's our answer:
// Iterate over all possible divisors of N.
// For each divisor K, check if all of the knights on the vertices are good, for each starting point
// in 0..K
//
// This last part of checking visits all of the vertices once, and is thus O(N)
// So the overall time complexity is O(N * divisors(N))
//
//
// My thoughts:
// Had the idea quickly, didn't see that it was fast enough instantly, but it was OK still.
// Implementation was OK, Rust is nice.
// Egor's lib has a very ergonomic way of finding divisors: `n`.divisors(). Pretty neat!
fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_long();
    let good = input.read_size_vec(n as usize);
    for d in n.divisors() {
        if d < 3 {
            continue;
        }

        let step_size = n as usize / d as usize;

        for i in 0..(step_size) {
            let all_good = good.iter().skip(i).step_by(step_size).all(|x| *x == 1);
            if all_good {
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
    let test_type = TestType::Single;
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
