//{"name":"B. Count the Number of Pairs","group":"Codeforces - Codeforces Round 855 (Div. 3)","url":"https://codeforces.com/contest/1800/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n11 2\naAaaBACacbE\n2 2\nab\n4 1\naaBB\n6 0\nabBAcC\n5 3\ncbccb\n","output":"5\n0\n1\n3\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BCountTheNumberOfPairs"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::StrReader;

type PreCalc = ();

// The crucial observation is that the string doesn't really matter, only the characters it
// contains
// For each letter, we already have some pairs ('a' and 'A' is already a pair that you do not want
// to mess), and (possibly) some leftover letters. For example, "aaaaA" has 1 pair, and 3 leftover
// 'a's. We can try to create new pairs with the leftovers.
// Because each pair is valued the same ('a' is the same worth as 'b' or 'c'), you can be greedy
// here and make whatever pairs you want, respecting K.
fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let (n, k) = (input.read_size(), input.read_size());
    let s = input.read_str();

    let mut answer = 0;
    let mut can_do_extra = 0;
    for letter in 'a'..='z' {
        let lower = s.iter().filter(|&x| x == letter as u8).count();
        let upper = s
            .iter()
            .filter(|&x| x == letter.to_ascii_uppercase() as u8)
            .count();

        let diff = lower.abs_diff(upper);
        answer += std::cmp::min(lower, upper);
        can_do_extra += diff / 2;
    }

    out.print_line(answer + std::cmp::min(can_do_extra, k));
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
