//{"name":"D. Lunar New Year and a Wander","group":"Codeforces - Codeforces Round 536 (Div. 2)","url":"https://codeforces.com/problemset/problem/1106/D","interactive":false,"timeLimit":3000,"tests":[{"input":"3 2\n1 2\n1 3\n","output":"1 2 3\n"},{"input":"5 5\n1 4\n3 4\n5 4\n3 2\n1 5\n","output":"1 4 3 2 5\n"},{"input":"10 10\n1 4\n6 8\n2 5\n3 7\n9 4\n5 6\n3 4\n8 10\n8 9\n1 10\n","output":"1 4 3 7 9 8 6 5 2 10\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DLunarNewYearAndAWander"}}}

use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use std::collections::BTreeSet;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let (n, m) = (input.read_size(), input.read_size());
    let edges = {
        let mut v = vec![];
        for _ in 0..m {
            let (a, b) = (input.read_size() - 1, input.read_size() - 1);
            v.push((a, b));
        }
        v
    };
    let graph = Graph::from_biedges(n, &edges);

    let mut order = vec![];
    let mut visited = vec![false; n];
    let mut reachable_vertices = BTreeSet::new();
    reachable_vertices.insert(0);
    loop {
        let next = reachable_vertices.pop_first().unwrap();
        visited[next] = true;

        order.push(next);

        for adjacent in &graph[next] {
            let to = adjacent.to();
            if !visited[to] {
                reachable_vertices.insert(to);
            }
        }

        if reachable_vertices.is_empty() {
            break;
        }
    }

    let order = order.into_iter().map(|v| v + 1).collect::<Vec<_>>();
    out.print_line(order);
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
