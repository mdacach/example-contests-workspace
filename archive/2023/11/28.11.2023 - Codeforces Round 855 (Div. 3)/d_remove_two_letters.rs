//{"name":"D. Remove Two Letters","group":"Codeforces - Codeforces Round 855 (Div. 3)","url":"https://codeforces.com/contest/1800/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"7\n6\naaabcc\n10\naaaaaaaaaa\n6\nabcdef\n7\nabacaba\n6\ncccfff\n4\nabba\n5\nababa\n","output":"4\n1\n5\n3\n3\n3\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DRemoveTwoLetters"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::StrReader;

type PreCalc = ();

// Let's simulate the process and see what happens:
// Suppose you have a 7-length string: a1 a2 a3 a4 a5 a6 a7
// Denote s[i] the string you get if you remove a_i and a_i+1
//
// s[1] = a3 a4 a5 a6 a7
// s[2] = a1 a4 a5 a6 a7
// s[3] = a1 a2 a5 a6 a7
// s[4] = a1 a2 a3 a6 a7
// s[5] = a1 a2 a3 a4 a7
// s[6] = a1 a2 a3 a4 a5
//
// Looking at s[1] and s[2], we see that the only differing character is in the first position.
// If a1 = a3, s[1] = s[2]
// Similarly,
// If a2 = a4, s[2] = s[3]
// If a3 = a5, s[3] = s[4]
// If a4 = a6, s[4] = s[5]
// ...
// This gives us an inkling of the answer - some consecutive strings will be equal.
// But is it possible for non consecutive strings to be equal too?
//
// Let's prove that it is not:
// Let's prove that if s[i] != s[i+1], then s[i] != s[i+k] - (if you want to have two strings
// equal, all of the strings in-between must be equal too).
//
// For s[i] to be equal s[i+k], the wholes string should match.
// In particular, let's look at the first i+k letters of both strings:
//
// 1. The first i+k letters in s[i+k] will be:
// s[i+k] -> a1 a2 a3 ...  a_i    a_i+1 a_i+2 ... a_i+k-1 a_i+k
// 2. The first i+k letters in s[i] will be:
// s[i] ->   a1 a2 a3 ... a_i+2   a_i+3 a_i+4 ... a_i+k-3 a_i+k-2
//                         ^^
//                       because a_i and a_i+1 were removed
// But because s[i] != s[i+1], we know that a_i and a_i+2 are different!
// Thus the first i+k letters of the two strings DO NOT MATCH - thus the whole string does not
// match.
//
// This means that the only equal strings must be consecutive!
// if s[1] != s[2], it will never be equal anyone else.
//
// Thus we have groups of equal strings, that count one to the answer.
//
fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_str();

    // let mut answer = 0;
    // for i in 0..(n - 2) {
    //     if s[i] != s[i + 2] {
    //         answer += 1;
    //     }
    // }
    // answer += 1;
    // out.print_line(answer);
    let mut answer = 1; // This is s[1]
    let mut different = 0;
    let mut last = 0;
    for i in 0..(n - 2) {
        if a[i] == a[i + 2] {
            // This means that s[i] = s[i + 1]
            // As we have already considered s[i], this doesn't change the answer.
        } else {
            // s[i + 1] is different
            answer += 1;
        }
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
