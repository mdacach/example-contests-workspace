//{"name":"D. Yet Another Monster Fight","group":"Codeforces - Educational Codeforces Round 158 (Rated for Div. 2)","url":"https://codeforces.com/contest/1901/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n2 1 5 6 4 3\n","output":"8\n"},{"input":"5\n4 4 4 4 4\n","output":"8\n"},{"input":"2\n1 1000000000\n","output":"1000000000\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DYetAnotherMonsterFight"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::multi_set::{self, MultiTreeSet};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let health = input.read_long_vec(n as usize);

    let mut attacked_from_left = vec![0; n];
    let mut attacked_from_right = vec![0; n];

    {
        attacked_from_left[0] = health[0];
        for i in 1..n {
            attacked_from_left[i] = health[i] + (i as i64);
        }
    }

    {
        attacked_from_right[n - 1] = health[n - 1];
        for i in (0..(n - 1)).rev() {
            attacked_from_right[i] = health[i] + (n as i64 - 1 - (i as i64));
        }
    }

    let mut answer = vec![i64::MAX; n];
    let mut needed = MultiTreeSet::new();
    for i in 0..n {
        needed.insert(attacked_from_left[i as usize]);
    }
    for position in 0..n {
        needed.remove(&attacked_from_left[position as usize]);
        let to_fix_others = *needed.last().unwrap_or(&0);
        let to_fix_this = health[position];

        answer[position] = std::cmp::max(to_fix_others, to_fix_this);

        needed.insert(attacked_from_right[position as usize]);
    }

    out.print_line(answer.into_iter().min().unwrap());
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
