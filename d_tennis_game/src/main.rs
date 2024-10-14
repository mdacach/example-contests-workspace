//{"name":"D. Tennis Game","group":"Codeforces - Codeforces Round 283 (Div. 2)","url":"https://codeforces.com/problemset/problem/496/D","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n1 2 1 2 1\n","output":"2\n1 3\n3 1\n"},{"input":"4\n1 1 1 1\n","output":"3\n1 4\n2 2\n4 1\n"},{"input":"4\n1 2 1 2\n","output":"0\n"},{"input":"8\n2 1 2 1 1 1 1 1\n","output":"3\n1 6\n2 3\n6 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DTennisGame"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let winner: Vec<usize> = input.read_vec(n);

    let mut player_one_points = vec![0; n];
    let mut player_two_points = vec![0; n];
    if winner[0] == 1 {
        player_one_points[0] = 1;
    } else {
        player_two_points[0] = 1;
    }
    for serve in 1..n {
        if winner[serve] == 1 {
            player_one_points[serve] = player_one_points[serve - 1] + 1;
            player_two_points[serve] = player_two_points[serve - 1];
        } else {
            player_two_points[serve] = player_two_points[serve - 1] + 1;
            player_one_points[serve] = player_one_points[serve - 1];
        }
    }

    let player_one_total_points = player_one_points[n - 1];
    let mut player_one_points_to_index = vec![0; player_one_total_points + 1];
    let mut current_points = 1;
    for i in 0..n {
        if player_one_points[i] == current_points {
            player_one_points_to_index[current_points] = i;
            current_points += 1;
        }
    }

    let player_two_total_points = player_two_points[n - 1];
    let mut player_two_points_to_index = vec![0; player_two_total_points + 1];
    let mut current_points = 1;
    for i in 0..n {
        if player_two_points[i] == current_points {
            player_two_points_to_index[current_points] = i;
            current_points += 1;
        }
    }

    let mut possible = vec![];
    for t in 1..=n {
        let mut player_one_sets = 0;
        let mut player_two_sets = 0;

        let mut player_one_current_points = 0;
        let mut player_two_current_points = 0;

        let mut current_index = 0;
        let mut last_winner = 0;
        loop {
            let one_next_points = player_one_current_points + t;
            let two_next_points = player_two_current_points + t;

            let one_index = if one_next_points > player_one_total_points {
                usize::MAX
            } else {
                player_one_points_to_index[one_next_points]
            };
            let two_index = if two_next_points > player_two_total_points {
                usize::MAX
            } else {
                player_two_points_to_index[two_next_points]
            };
            if one_index == two_index {
                if current_index == n && last_winner == winner[n - 1] {
                    if last_winner == 1 {
                        if player_one_sets > player_two_sets {
                            possible.push((player_one_sets, t));
                        }
                    } else {
                        if player_two_sets > player_one_sets {
                            possible.push((player_two_sets, t));
                        }
                    }
                }
                break;
            }

            if one_index < two_index {
                player_one_sets += 1;
                player_one_current_points += t;
                player_two_current_points += player_two_points[one_index];
                if current_index > 0 {
                    player_two_current_points -= player_two_points[current_index - 1];
                }
                current_index = one_index;
                last_winner = 1;
            } else {
                player_two_sets += 1;
                player_two_current_points += t;
                player_one_current_points += player_one_points[two_index];
                if current_index > 0 {
                    player_one_current_points -= player_one_points[current_index - 1];
                }
                current_index = two_index;
                last_winner = 2;
            }
            current_index += 1;
        }
    }

    possible.sort();
    out_line!(possible.len());
    for (s, t) in possible {
        out_line!(s, t);
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
