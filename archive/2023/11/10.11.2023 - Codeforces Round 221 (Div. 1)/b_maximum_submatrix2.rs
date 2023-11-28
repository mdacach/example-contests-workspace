//{"name":"B. Maximum Submatrix 2","group":"Codeforces - Codeforces Round 221 (Div. 1)","url":"https://codeforces.com/problemset/problem/375/B","interactive":false,"timeLimit":2000,"tests":[{"input":"1 1\n1\n","output":"1\n"},{"input":"2 2\n10\n11\n","output":"2\n"},{"input":"4 3\n100\n011\n000\n101\n","output":"2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BMaximumSubmatrix2"}}}

use algo_lib::collections::md_arr::arr2d::{Arr2d, Arr2dRead};
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

// Our desired submatrix must start at some column, and then go to the right.
// Let's iterate over this inital column.
//
// If you have a fixed initial column, you know that
// A. You want to pick many rows with 1s at this column
//    - Otherwise you can't use them in your submatrix
// B. You also want to pick rows with many 1s to the right
//    - Because you want to create a big submatrix, extending to the right
//
// Let's greedily pick rows. The row with more ones to the right (including the initial-column one)
// is better, because it may help us create the biggest submatrix possible.
// Then, let's sort the rows by "rightmost ones", and try to construct a submatrix with them.
//
// By moving from row i to row i+i, the amount of ones-to-the-right (width of your rectangle) may decrease,
// but your height increases, so it's not clear when you should stop adding new rows.
// But we can just try them all:
// For each row i, the size of the rectangle ending at it is its height (i+1) times its width
// (ones_to_the_right[i]).
//
// To keep track of ones_to_the_right[i], you can iterate right to left.
//
// Note that you can just sort normally (sorting library should be quite quick), but you could also
// use some O(N) sorting algorithm if worried.
//
// My thoughts:
// This problem looked tricky at the start. I was thinking about some DP, and this problem made me
// recall two famous "DP" problems: Largest Rectangle with 1s, and Largest Rectangle in Histogram.
// Although they wouldn't help, I took a detour and refreshed on them both, and it was pretty nice.
//
// Apart from the initial detour, I had the idea somewhat quickly afterwards. Implementation was
// easy too.
fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let (n, m) = (input.read_size(), input.read_size());
    let table = input.read_char_table(n, m);

    let mut ones_to_the_right = Arr2d::new(n, m, 0_usize);
    for i in 0..n {
        let mut current_count = 0;
        for j in (0..m).rev() {
            if table[i][j] == '1' {
                current_count += 1;
            } else {
                current_count = 0;
            }
            ones_to_the_right[i][j] = current_count;
        }
    }

    let mut best_answer = 0_usize;
    for starting_column in 0..m {
        let mut all_values = vec![];
        for row in 0..n {
            all_values.push(ones_to_the_right[row][starting_column]);
        }
        all_values.sort_unstable_by(|a, b| b.cmp(a));

        for (i, value) in all_values.into_iter().enumerate() {
            let answer_here = (i + 1) * value;
            best_answer.maxim(answer_here);
        }
    }

    out.print_line(best_answer);
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
