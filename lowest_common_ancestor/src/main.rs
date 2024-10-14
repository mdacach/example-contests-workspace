//{"name":"Lowest Common Ancestor","group":"Library Checker","url":"https://judge.yosupo.jp/problem/lca","interactive":false,"timeLimit":5000,"tests":[],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"LowestCommonAncestor"}}}

use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let q = input.read_size();
    let mut graph = Graph::new(n);
    for i in 1..n {
        let p = input.read_size();
        graph.add_edge(i, p);
        graph.add_edge(p, i);
    }
    let root = 0;
    let lca = graph.lowest_common_ancestor_table(root);
    for i in 0..q {
        let (a, b) = (input.read_size(), input.read_size());
        out_line!(lca.of(a, b));
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
