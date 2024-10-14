//{"name":"D. Red-Green Towers","group":"Codeforces - Codeforces Round 273 (Div. 2)","url":"https://codeforces.com/problemset/problem/478/D","interactive":false,"timeLimit":2000,"tests":[{"input":"4 6\n","output":"2\n"},{"input":"9 7\n","output":"6\n"},{"input":"1 1\n","output":"2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DRedGreenTowers"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

const MOD: u64 = 1e9 as u64 + 7;
const MAX_HEIGHT: usize = 895;

fn solve(input: &mut Input, _test_case: usize) {
    let (r, g) = (input.read_size(), input.read_size());
    let mut current_ways = vec![0_u64; r + 1];
    let mut previous_ways = vec![0_u64; r + 1];
    previous_ways[0] = 1;

    let mut total_used = 0;
    let mut greatest_height = 0;
    'outer: for level in 1..MAX_HEIGHT {
        let mut possible = false;
        for reds_used in 0..=(std::cmp::min(total_used, r)) {
            let greens_used = total_used - reds_used;
            if previous_ways[reds_used] == 0 {
                continue;
            }

            // Put green at this level
            if greens_used + level <= g {
                current_ways[reds_used] += previous_ways[reds_used];
                current_ways[reds_used] %= MOD;

                possible = true;
            }
            // Put red at this level
            if reds_used + level <= r {
                current_ways[reds_used + level] += previous_ways[reds_used];
                current_ways[reds_used + level] %= MOD;

                possible = true;
            }
        }

        if !possible {
            break 'outer;
        }

        std::mem::swap(&mut previous_ways, &mut current_ways);
        current_ways.fill(0);

        total_used += level;
    }
    let all_ways = previous_ways
        .iter()
        .copied()
        .reduce(|acc, e| (acc + e) % MOD)
        .unwrap();
    out_line!(all_ways);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
    output().flush();
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
