//{"name":"C. Read Time","group":"Codeforces - Codeforces Round 200 (Div. 1)","url":"https://codeforces.com/problemset/problem/343/C","interactive":false,"timeLimit":1000,"tests":[{"input":"3 4\n2 5 6\n1 3 6 8\n","output":"2\n"},{"input":"3 3\n1 2 3\n1 2 3\n","output":"0\n"},{"input":"1 2\n165\n142 200\n","output":"81\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CReadTime"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let (n, m) = (input.read_size(), input.read_size());
    let mut heads = input.read_size_vec(n);
    let mut tracks = input.read_size_vec(m);

    heads.sort();
    tracks.sort();

    let check = |time: usize| -> bool {
        let mut current_track_index = 0;
        for &current_head in &heads {
            let current_track = tracks[current_track_index];
            let mut distance = 0;
            let mut remaining_time = time;
            // To the left
            if current_track <= current_head {
                distance = current_track.abs_diff(current_head);
                if current_track.abs_diff(current_head) > time {
                    return false;
                }
                if tracks[current_track_index] <= current_head {
                    while current_track_index < m && tracks[current_track_index] <= current_head {
                        current_track_index += 1;
                    }
                }
                if current_track_index >= m {
                    return true;
                }
                remaining_time = time - distance;
            }

            // To the right
            let possible = |track: usize| {
                let this_diff = track.abs_diff(current_head);
                let extra = std::cmp::min(this_diff, distance);
                extra + this_diff <= remaining_time
            };

            while current_track_index < m && possible(tracks[current_track_index]) {
                current_track_index += 1;
            }
            if current_track_index >= m {
                return true;
            }
        }
        false
    };

    let mut left = 0;
    let mut right = 1e18 as usize;
    while right - left > 1 {
        let middle = left + (right - left) / 2;
        if check(middle) {
            right = middle;
        } else {
            left = middle;
        }
    }

    // Handle 0
    if check(left) {
        out_line!(left);
    } else {
        out_line!(right);
    }
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
