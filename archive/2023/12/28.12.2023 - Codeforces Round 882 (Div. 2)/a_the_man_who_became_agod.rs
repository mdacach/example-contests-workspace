//{"name":"A. The Man who became a God","group":"Codeforces - Codeforces Round 882 (Div. 2)","url":"https://codeforces.com/contest/1847/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n4 2\n1 3 5 2\n6 3\n1 9 12 4 7 2\n12 8\n1 9 8 2 3 3 1 8 7 7 9 2\n","output":"4\n11\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ATheManWhoBecameAGod"}}}

use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::collections::slice_ext::consecutive_iter::ConsecutiveIter;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

// Consider these suspicions:
// 1   5   10   5   20   15
// If we keep all villagers in the same group, we will have the power be
//   4 + 5 + 5 + 15 + 5 = 34
// What happens if we split into two groups, with some boundary?
// 1 5 10 | 5 20 15
// 4 + 5    15 + 5
// We see that a single summand has disappeared, and every other one is kept the same.
// Generalizing this, we can see that by choosing the boundary of splitting into groups, we are
// choosing which summand to get rid of. Then, greedily, we always want to remove the greater
// summands, because we want the sum to be the smallest.
//
// In this example, with k = 3, we can add two boundaries (to make three groups in the end), and
// should remove a 15 and a 5
//
// (1 5 10) (5) (20 15) works.
fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let (n, k) = (input.read_size(), input.read_size());
    let values = input.read_size_vec(n);
    let mut sums = values
        .consecutive_iter()
        .map(|(a, b)| a.abs_diff(*b))
        .collect_vec();

    sums.sort_unstable();
    let answer: usize = sums.into_iter().take(n - k).sum();
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
