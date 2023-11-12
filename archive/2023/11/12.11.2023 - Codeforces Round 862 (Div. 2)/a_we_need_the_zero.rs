//{"name":"A. We Need the Zero","group":"Codeforces - Codeforces Round 862 (Div. 2)","url":"https://codeforces.com/contest/1805/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n3\n1 2 5\n3\n1 2 3\n4\n0 1 2 3\n4\n1 2 2 3\n1\n1\n","output":"6\n0\n3\n-1\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AWeNeedTheZero"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

// The XOR operation is interesting because it often "cancels" itself.
// If you do A XOR A, the answer is 0.
//
// Let's look at what happens with a simple array: [A1, A2].
// Let V be the value we have chosen.
//
//     B1     XOR     B2
// (A1 XOR V) XOR (A2 XOR V)
//
// We can safely rearrange the operations:
// (A1 XOR A2) XOR (V XOR V)
//                     ^ but this is 0!
//
// So, for even-length arrays, the XOR operations cancel each other and we are left with
// A1 XOR A2 (Generalizing, the XOR of all numbers in the array.)
//
// That is, we have NO CONTROL over the final output.
//
// For odd-length arrays, most XOR operations will cancel each other, but ONE.
// Se we will have (A1 XOR A2 XOR A3 ...) XOR V.
// You can always make it equal 0 by picking V as the same of the left-hand side.
//
//
//
// My thoughts:
// Interesting problem, maybe too math-heavy for an A? In any case, nice one.
// I didn't see that the bounds were small (and we could brute-force it), so I must pay more
// attention to it in easier problems.
fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let values = input.read_size_vec(n);

    let mut all_xor = 0;
    for &x in &values {
        all_xor ^= x;
    }

    if values.len() % 2 == 0 {
        if all_xor == 0 {
            out.print_line(0);
        } else {
            out.print_line(-1);
        }
    } else {
        out.print_line(all_xor);
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
