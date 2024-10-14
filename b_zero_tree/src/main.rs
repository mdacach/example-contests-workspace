//{"name":"B. Zero Tree","group":"Codeforces - Codeforces Round 168 (Div. 1)","url":"https://codeforces.com/problemset/problem/274/B","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n1 2\n1 3\n1 -1 1\n","output":"3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BZeroTree"}}}

use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let edges = input.read_size_pair_vec(n - 1).dec();
    let mut tree = Graph::new(n);
    for &(u, v) in &edges {
        tree.add_edge(u, BiEdge::new(v));
    }

    let mut must_pass_positive = vec![0; n];
    let mut must_pass_negative = vec![0; n];
    let mut dfs = RecursiveFunction2::new(|f, current: usize, parent: usize| {
        for child in &tree[current] {
            if child.to() == parent {
                continue;
            }
            f.call(child.to(), current);
        }
    });

    dfs.call(0, 0);
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
