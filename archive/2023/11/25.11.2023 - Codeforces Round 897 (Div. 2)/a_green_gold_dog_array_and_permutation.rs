//{"name":"A. green_gold_dog, array and permutation","group":"Codeforces - Codeforces Round 897 (Div. 2)","url":"https://codeforces.com/contest/1867/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n1\n100000\n2\n1 1\n3\n10 3 3\n","output":"1\n2 1\n1 3 2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AGreenGoldDogArrayAndPermutation"}}}

use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

// If we can find an optimal construction, we don't need to bother with any additional details.
// Say we find a construction that has all the element-wise differences to be different. This is
// the best we can do, so we don't need to go looking at any other solutions. Note that this wouldn't
// be the case if all _but one_ were different - it would be possible there's another better
// solution, then.
//
// So the trick here is to test some stuff.
// First, observe that we are free to put the numbers wherever (because we have full control over
// the permutation). So we can always pair elements arbitraritly. Equivalently, we can think of
// this as "we can sort the array a in whichever way we want".
// Let's sort it non-decreasing.
//
// What happens if we pair up the smallest elements at a time?
// a = 3 5 7 8
// p = 1 2 3 4
// c = 2 3 4 4
//           ^ not great
// It can be even worse,
// a = 1 2 3 4
// p = 1 2 3 4
// all the same.
//
// So that doesn't work. Let's try the other way around - pair the biggest with the smallest
// a =  3  5  7  8
// p =  4  3  2  1
// c = -1  2  5  7
// that's very good! that's optimal in this case.
// In fact, we can prove it's always optimal.
// The elements in c always increase by at least 1, so they will always be different.
//
// c_i = a_i - p_i
// c_i+1 = a_i+1 - p_i+1
// We know that a_i+1 > a_i. Let's say a_i+1 = a_i + A
// We know that p_i+1 = p_i - 1. (Because it's a permutation).
// So, c_i+1 = (a_i + A) - (p_i - 1)
//     c_i+1 = (a_i - p_i) + (A + 1) -> note the sign has changed on -1.
//     c_i+1 = c_i + (A + 1)
//                      ^ always positive.
//
// Then we have found an optimal construction, and don't need to look any further.
//
//
//
//
// Thoughts:
// This is a problem where experience really helps - it's a Div2A, so the solution must be
// something simple to think of and to code. So that basically leaves us with the two pairings
// above. Interestingly, I thought of the wrong one first.
fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let values = input.read_size_vec(n);

    let mut biggest_indexes = (0_usize..n).collect_vec();
    biggest_indexes.sort_unstable_by(|a, b| values[*b].cmp(&values[*a]));

    let mut answer = vec![0; n];
    let mut current = 1;
    for id in biggest_indexes {
        answer[id] = current;
        current += 1;
    }
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
