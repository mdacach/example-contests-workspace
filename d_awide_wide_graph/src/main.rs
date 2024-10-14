//{"name":"D. A Wide, Wide Graph","group":"Codeforces - Codeforces Round 862 (Div. 2)","url":"https://codeforces.com/contest/1805/problem/D","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n1 2\n1 3\n2 4\n2 5\n3 6\n","output":"1 1 2 4 6 6\n"},{"input":"5\n1 2\n2 3\n3 4\n3 5\n","output":"1 1 3 5 5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DAWideWideGraph"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let edges = input.read_size_pair_vec(n - 1).dec();
    let tree = Graph::from_biedges(n, &edges);

    let mut max_dist_in_subtree = vec![0; n];
    let mut other_max_dist_in_subtree = vec![0; n];
    let mut solve_subtree = RecursiveFunction2::new(|f, current: usize, parent: usize| {
        for child in &tree[current] {
            let child = child.to();
            if child == parent {
                continue;
            }
            f.call(child, current);

            let answer_here = max_dist_in_subtree[child] + 1;
            if answer_here > max_dist_in_subtree[current] {
                other_max_dist_in_subtree[current] = max_dist_in_subtree[current];
                max_dist_in_subtree[current] = answer_here;
            } else if answer_here > other_max_dist_in_subtree[current] {
                other_max_dist_in_subtree[current] = answer_here;
            }
        }
    });
    solve_subtree.call(0, 0);

    let mut max_dist_outside_subtree = vec![0; n];
    let mut solve_outside_subtree = RecursiveFunction2::new(|f, current: usize, parent: usize| {
        for child in &tree[current] {
            let child = child.to();
            if child == parent {
                continue;
            }

            max_dist_outside_subtree[child] = max_dist_outside_subtree[current] + 1;
            if max_dist_in_subtree[current] == max_dist_in_subtree[child] + 1 {
                max_dist_outside_subtree[child].maxim(other_max_dist_in_subtree[current] + 1);
            } else {
                max_dist_outside_subtree[child].maxim(max_dist_in_subtree[current] + 1);
            }

            f.call(child, current);
        }
    });
    solve_outside_subtree.call(0, 0);

    let mut max_dist_to_other = vec![0; n];
    for v in 0..n {
        max_dist_to_other[v] = std::cmp::max(max_dist_in_subtree[v], max_dist_outside_subtree[v]);
    }

    let mut becomes_separate = vec![0; n + 1];
    for v in 0..n {
        becomes_separate[max_dist_to_other[v] + 1] += 1;
    }

    let mut currently_separate = 0;
    for i in 1..=n {
        currently_separate += becomes_separate[i];
        out.print(std::cmp::min(1 + currently_separate, n));
        out.print(" ");
    }
    out.print_line("");
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
