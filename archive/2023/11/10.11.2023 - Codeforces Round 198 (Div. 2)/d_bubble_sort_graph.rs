//{"name":"D. Bubble Sort Graph","group":"Codeforces - Codeforces Round 198 (Div. 2)","url":"https://codeforces.com/problemset/problem/340/D","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n3 1 2\n","output":"2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DBubbleSortGraph"}}}

use algo_lib::dp::lis::longest_increasing_subsequence;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

// This problem is quite tricky.
//
// Let's look at an example independent set.
// Suppose 1, 2 is an independent set in the graph.
// This means that there must be no edge between 1 and 2.
// In our Bubble Sort algorithm, if the permutation looks like x x x 2 x 1 x
// We know that 1 and 2 will get swapped (because they are in the wrong order)
// Then: we will have an edge in the graph - independent set is broken.
// So the only way that 1 and 2 may be in an independent set, is if they are in the right relative
// order in the permutation.
//
// Now let's cheat a bit and become real smart for a moment.
// Consider the Longest Increasing Subsequence of the permutation.
// This is the longest sequence of numbers in the right relative order.
// They won't have any edge between them.
// We _could_ pick this as our independent set.
//
// But is it enough? Is it the _maximum_ independent set?
// It actually is, and you can reason about it this way:
//
// Suppose there's an even better independent set, a bigger one than our LIS.
// But then, because all of those are in an independent set, there's no edge between any pair of
// those numbers.
// If there's no edge between any pair, they are in the right relative order (otherwise BubbleSort
// would have swapped them).
// If they are in the right relative order, they COULD BE AN INCREASING SUBSEQUENCE.
//
// So our LIS would consider that subsequence, and as we are picking the Longest one, we are
// picking the same answer or better.
//
//
//
//
// My thoughts:
// My write-up sucks a little, I should probably clear it up... but the editorial is pretty good
// already.
// I was absolutely defeated by this problem, and why it is rated 1500 escapes me.
// I thought about it hard, but didn't think of LIS. This is one aspect I struggle - linking known
// DP solutions to other problems, but I am improving at it. Interestingly I don't have the same
// problem with non-DP.
//
// A beautiful problem with a beautiful solution, though.
fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let values = input.read_long_vec(n);

    let answer = longest_increasing_subsequence(&values);
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
