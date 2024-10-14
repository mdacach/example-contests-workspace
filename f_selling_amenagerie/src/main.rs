//{"name":"F. Selling a Menagerie","group":"Codeforces - Codeforces Round 895 (Div. 3)","url":"https://codeforces.com/contest/1872/problem/F","interactive":false,"timeLimit":2000,"tests":[{"input":"8\n3\n2 3 2\n6 6 1\n8\n2 1 4 3 6 5 8 7\n1 2 1 2 2 1 2 1\n5\n2 1 1 1 1\n9 8 1 1 1\n2\n2 1\n1000000000 999999999\n7\n2 3 2 6 4 4 3\n1 2 3 4 5 6 7\n5\n3 4 4 1 3\n3 4 5 6 7\n3\n2 1 1\n1 2 2\n4\n2 1 4 1\n1 1 1 1\n","output":"1 2 3\n2 4 5 1 6 3 7 8\n3 4 5 1 2\n1 2\n7 5 1 3 2 6 4\n5 3 2 4 1\n3 2 1\n3 4 1 2\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FSellingAMenagerie"}}}

use std::collections::VecDeque;
use std::thread;
use std::time::Duration;

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let mut afraid_of = vec![0; n];
    let mut indegree = vec![0; n];
    for i in 0..n {
        let other = input.read_size() - 1;
        afraid_of[i] = other;
        indegree[other] += 1;
    }
    let cost = input.read_int_vec(n);

    let mut sorted_by_cost = Vec::new();
    for i in 0..n {
        sorted_by_cost.push(i);
    }
    sorted_by_cost.sort_by(|a, b| cost[*b].cmp(&cost[*a]));

    let mut toposort_queue = VecDeque::new();
    indegree
        .iter()
        .enumerate()
        .filter(|(i, c)| **c == 0)
        .for_each(|(i, c)| toposort_queue.push_back(i));
    let mut processed = vec![false; n];

    let mut answer = Vec::new();

    while let Some(current_node) = toposort_queue.pop_front() {
        if processed[current_node] {
            continue;
        }
        // Pop the current node -> sell it
        processed[current_node] = true;
        answer.push(current_node + 1);

        let other = afraid_of[current_node];
        indegree[other] -= 1;
        if indegree[other] == 0 {
            toposort_queue.push_back(other);
        }
    }

    for current_vertex in 0..n {
        if processed[current_vertex] {
            continue;
        }

        let mut cycle = Vec::new();
        cycle.push(current_vertex);
        let mut next = afraid_of[current_vertex];
        processed[current_vertex] = true;
        while !processed[next] {
            cycle.push(next);
            processed[next] = true;
            next = afraid_of[next];
        }

        let should_become_last = cycle
            .clone()
            .iter()
            .enumerate()
            .min_by(|a, b| {
                let cost_a = cost[*a.1];
                let cost_b = cost[*b.1];
                cost_a.cmp(&cost_b)
            })
            .unwrap()
            .0;

        let len = cycle.len();
        cycle.rotate_right(len - 1 - should_become_last);
        cycle.iter_mut().for_each(|x| *x += 1);
        answer.extend(cycle);
    }

    out_line!(answer);
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
