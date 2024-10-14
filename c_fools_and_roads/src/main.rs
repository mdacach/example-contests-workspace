//{"name":"C. Fools and Roads","group":"Codeforces - Codeforces Round 121 (Div. 1)","url":"https://codeforces.com/problemset/problem/191/C","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n1 2\n1 3\n2 4\n2 5\n2\n1 4\n3 5\n","output":"2 1 1 1\n"},{"input":"5\n3 4\n4 5\n1 4\n2 4\n3\n2 3\n1 3\n3 5\n","output":"3 1 1 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CFoolsAndRoads"}}}

use algo_lib::collections::array2d::Array2d;
use algo_lib::collections::iter_ext::{IterExt, IterOrdExt};
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{
    Callable3, Callable4, RecursiveFunction2, RecursiveFunction3, RecursiveFunction4,
};
use algo_lib::{out, out_line};
use std::collections::HashMap;

const NO_PARENT: usize = usize::MAX;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let mut graph = Graph::new(n);
    let mut edges = vec![];
    for _ in 0..n - 1 {
        let (a, b) = (input.read_size(), input.read_size());
        graph.add_edge(a - 1, b - 1);
        graph.add_edge(b - 1, a - 1);
        edges.push((a - 1, b - 1));
    }

    let lca = graph.lowest_common_ancestor_table(0);

    let mut level = vec![0; n];
    let mut up = vec![0; n];
    let mut dfs = RecursiveFunction3::new(|f, current, parent, current_level| {
        level[current] = current_level;
        up[current] = parent;
        for &child in &graph.adjacency_list[current] {
            if child == parent {
                continue;
            }
            f.call(child, current, current_level + 1);
        }
    });
    dfs.call(0, 0, 0);

    let mut ordering = (0..n).collect_vec();
    ordering.sort_by(|a, b| level[*b].cmp(&level[*a]));

    let mut marked = vec![0; n];
    let k = input.read_size();
    for _ in 0..k {
        let (l, r) = (input.read_size(), input.read_size());
        marked[l - 1] += 1;
        marked[r - 1] += 1;
        let lca = lca.of(l - 1, r - 1);
        marked[lca] -= 2;
    }

    let mut count_node = vec![0; n];
    for node in ordering {
        if node == 0 {
            continue;
        }

        let next = up[node];
        count_node[next] += count_node[node] + marked[node];
    }

    for (a, b) in edges {
        if level[a] > level[b] {
            out!(count_node[a] + marked[a], " ");
        } else {
            out!(count_node[b] + marked[b], " ");
        }
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
