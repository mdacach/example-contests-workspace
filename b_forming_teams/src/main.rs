//{"name":"B. Forming Teams","group":"Codeforces - Codeforces Round 133 (Div. 2)","url":"https://codeforces.com/problemset/problem/216/b","interactive":false,"timeLimit":2000,"tests":[{"input":"5 4\n1 2\n2 4\n5 3\n1 4\n","output":"1\n"},{"input":"6 2\n1 4\n3 4\n","output":"0\n"},{"input":"6 6\n1 2\n2 3\n3 1\n4 5\n5 6\n6 4\n","output":"2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BFormingTeams"}}}

use algo_lib::graph::edges::edge::Edge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::graph::strongly_connected_components::StronglyConnectedComponents;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction, RecursiveFunction2};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let mut graph = Graph::new(n);
    for _ in 0..m {
        let (a, b) = (input.read_size() - 1, input.read_size() - 1);
        graph.add_edge(a, Edge::new(b));
        graph.add_edge(b, Edge::new(a));
    }

    let mut visited = vec![false; n];

    let mut current_size = 0;
    let mut has_cycle = false; // Note this is special because of problem's constraints.

    let mut total_people = 0;
    let mut to_remove = 0;
    for v in 0..n {
        if !visited[v] {
            current_size = 0;
            has_cycle = false;

            let mut dfs = RecursiveFunction2::new(|f, current: usize, parent: usize| {
                visited[current] = true;
                current_size += 1;
                for child in &graph[current] {
                    if child.to() == parent {
                        continue;
                    }

                    if visited[child.to()] {
                        has_cycle = true;
                        return;
                    }

                    f.call(child.to(), current);
                }
            });
            dfs.call(v, v);

            if has_cycle && current_size % 2 == 1 {
                current_size -= 1;
                to_remove += 1;
            }

            total_people += current_size;
        }
    }

    if total_people % 2 == 1 {
        to_remove += 1;
    }
    out.print_line(to_remove);
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
