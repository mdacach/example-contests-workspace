//{"name":"B. Psychos in a Line","group":"Codeforces - Codeforces Round 189 (Div. 1)","url":"https://codeforces.com/problemset/problem/319/B","interactive":false,"timeLimit":1000,"tests":[{"input":"10\n10 9 7 8 6 5 3 4 2 1\n","output":"2\n"},{"input":"6\n1 2 3 4 5 6\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BPsychosInALine"}}}

use std::collections::VecDeque;

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let ids = input.read_size_vec(n);

    let mut steps_to_kill = vec![0; n];
    {
        let mut stack = VecDeque::new();
        let mut current_index = 0;
        for &current_id in &ids {
            let mut max_from_smaller = 0;
            while let Some(&previous_index) = stack.back() {
                let previous_id = ids[previous_index];
                if previous_id < current_id {
                    stack.pop_back();
                    max_from_smaller.maxim(steps_to_kill[previous_index]);
                } else {
                    steps_to_kill[current_index] = max_from_smaller + 1;
                    break;
                }
            }
            stack.push_back(current_index);
            current_index += 1;
        }
    };

    let most_steps = steps_to_kill.iter().max().unwrap();
    out_line!(most_steps);
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
