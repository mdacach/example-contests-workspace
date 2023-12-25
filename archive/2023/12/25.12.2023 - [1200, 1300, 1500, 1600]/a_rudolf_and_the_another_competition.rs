//{"name":"A. Rudolf and the Another Competition","group":"Codeforces - [1200, 1300, 1500, 1600]","url":"https://codeforces.com/group/QsHQUUet4f/contest/489749/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n3 3 120\n20 15 110\n90 90 100\n40 40 40\n2 1 120\n30\n30\n1 3 120\n10 20 30\n3 2 27\n8 9\n10 7\n10 8\n3 3 15\n7 2 6\n7 5 4\n1 9 8\n","output":"2\n1\n1\n2\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ARudolfAndTheAnotherCompetition"}}}

use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::collections::md_arr::arr2d::{Arr2d, Arr2dRead};
use algo_lib::io::input::{Input, Readable};
use algo_lib::io::output::Output;

type PreCalc = ();

// We must first understand what is the optimal order for contestants to solve the problems in.
// Note that we only care about the time where we _complete_ the problem.
// This is also known as [turnaround time](https://en.wikipedia.org/wiki/Turnaround_time).
// Minimizing the turnaround time is a scheduling theory question.
// In this problem, the task is much easier - because all the jobs arrive at the same order (you
// know all the problems already) and each duration is known (the AI tells you how much time it
// will take), the solution is to use "Shortest Job First" - aka greedy.
//
// Now that we know the optimal order, it's just a matter of simulating the process.
// Compute the total penalty for each contestant, and rank Rudolph among them.
// Implementation is not very interesting.
//
//
// Thoughts: Nice problem hinting at Scheduling Theory. You of course don't need to know that to
// solve this problem, but it's a nice background.
fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let (n, m, h) = (input.read_size(), input.read_size(), input.read_size());
    let times = input.read_size_table(n, m);

    let mut rudolf = (0, 0);
    let mut ahead = 0;
    for participant in 0..n {
        let mut this_times = times.row(participant).collect_vec();
        this_times.sort_unstable();

        let mut total_penalty = 0;
        let mut current_time = 0;
        let mut points = 0;
        for problem in this_times {
            if problem + current_time <= h {
                current_time += problem;
                total_penalty += current_time;
                points += 1;
            } else {
                break;
            }
        }

        if participant == 0 {
            rudolf = (points, total_penalty);
        } else {
            let (my_points, my_penalty) = rudolf;
            if points > my_points {
                ahead += 1;
            } else if points == my_points && total_penalty < my_penalty {
                ahead += 1;
            }
        }
    }

    out.print_line(ahead + 1);
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
