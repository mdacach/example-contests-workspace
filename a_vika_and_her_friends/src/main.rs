//{"name":"A. Vika and Her Friends","group":"Codeforces - Codeforces Round 885 (Div. 2)","url":"https://codeforces.com/contest/1848/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n2 2 1\n1 1\n1 2\n2 2 2\n1 1\n2 2\n2 2\n1 2 1\n1 1\n1 2\n5 5 4\n3 3\n1 1\n1 5\n5 1\n5 5\n2 2 2\n1 1\n2 1\n1 2\n3 4 1\n1 2\n3 3\n","output":"YES\nNO\nYES\nNO\nYES\nYES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AVikaAndHerFriends"}}}

use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

// I just guessed it, sorry. Intuitively, because the friend knows where Vika goes, they can always
// "get closer", I guess...
// The chess coloring argument from the editorial is neat, though. I am still to really read the
// other one...
//
//
// My thoughts:
// Initially misread the statement as being only one minute (which would make sense for div2 A!).
// So I lost some time there. After that it was OK - could definitely been faster, but whatever,
// hard probs.
fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let (n, m, k) = (input.read_size(), input.read_size(), input.read_size());
    let mut grid = Arr2d::new(n + 2, m + 2, false);
    let (start_x, start_y) = (input.read_size(), input.read_size());

    let mut bad = false;
    for _ in 0..k {
        let (x, y) = (input.read_size(), input.read_size());

        bad = bad || (x.abs_diff(start_x) + y.abs_diff(start_y)) % 2 == 0;
    }

    if bad {
        out.print_line("NO");
    } else {
        out.print_line("YES");
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
