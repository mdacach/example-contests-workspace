//{"name":"B. The Corridor or There and Back Again","group":"Codeforces - Codeforces Round 895 (Div. 3)","url":"https://codeforces.com/contest/1872/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"7\n1\n2 2\n3\n2 8\n4 3\n5 2\n1\n200 200\n4\n1 20\n5 9\n3 179\n100 1\n2\n10 1\n1 18\n2\n1 1\n1 2\n3\n1 3\n1 1\n1 3\n","output":"2\n5\n299\n9\n9\n1\n1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BTheCorridorOrThereAndBackAgain"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_int();
    let mut time_left: Vec<i32> = vec![10000; 300];
    (0..n).for_each(|_| {
        let room = input.read_size();
        let time = input.read_int();
        time_left[room] = time_left[room].min(time);
    });

    let mut current_room = 1;
    let mut can_go = (time_left[current_room] - 1) / 2;
    for i in 1..=20000 {
        if can_go == 0 {
            break;
        }
        current_room += 1;
        can_go -= 1;
        can_go.minim((time_left[current_room] - 1) / 2);
    }
    out_line!(current_room);
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
    }
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
