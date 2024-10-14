//{"name":"B. Vika and the Bridge","group":"Codeforces - Codeforces Round 885 (Div. 2)","url":"https://codeforces.com/contest/1848/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n5 2\n1 1 2 1 1\n7 3\n1 2 3 3 3 2 1\n6 6\n1 2 3 4 5 6\n8 4\n1 2 3 4 2 3 1 4\n3 1\n1 1 1\n","output":"0\n1\n2\n2\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BVikaAndTheBridge"}}}

use std::collections::BTreeMap;

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

// When you choose the color you are going to walk, you don't care about the others.
// Consider only the planks of color K. You need to know how far away they are of each other, so
// you need to keep track of their positions in the array.
// Now you have an array representing where you are going to step in
// K = {2, 4, 7, 8, 9}
// From this, you know what's the maximum step length you will need - from 4 to 7.
// So let's repaint the middle plank of that segment to color K, decreasing our step length.
// You always want to repaint the middle such plank, netting you two smaller steps.
// The new array would look like:
// K': {2, 4, 5, 7, 8, 9}
//            ^
//       could also be 6
// And now you want to check what's the biggest step in this new array.
// Do this for each color.
//
// Note that you don't need to do that exactly - you don't need to recreate the new array.
// If you keep track of only the step lengths, you know which one you are removing and which two
// you are adding.
//
//
//
//
// My thoughts:
// Good question, maybe a tiny bit too hard to be 1200, but it's OK.
// Had the idea fast, implemented fast, all's good.
fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let (n, _k) = (input.read_size(), input.read_size());
    let bridges = input.read_size_vec(n);

    let mut positions: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    for i in 0..n {
        let entry = positions.entry(bridges[i]).or_default();
        entry.push(i + 1);
    }

    let mut best_answer = usize::MAX;
    for (_color, positions) in positions.into_iter() {
        let to_last = n - positions.last().unwrap();
        let mut max_diff = to_last;
        let mut second_max_diff = 0;
        let mut previous = None;

        for p in positions {
            let diff_to_previous = if let Some(previous) = previous {
                p - previous - 1
            } else {
                p - 1
            };
            if diff_to_previous >= max_diff {
                second_max_diff = max_diff;
                max_diff = diff_to_previous;
            } else if diff_to_previous > second_max_diff {
                second_max_diff = diff_to_previous;
            }

            previous = Some(p);
        }

        let new_max_diff = if max_diff % 2 == 1 {
            max_diff / 2
        } else {
            max_diff / 2
        };

        let answer_here = std::cmp::max(new_max_diff, second_max_diff);
        best_answer.minim(answer_here);
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
