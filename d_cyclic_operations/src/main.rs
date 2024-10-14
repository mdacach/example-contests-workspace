//{"name":"D. Cyclic Operations","group":"Codeforces - Codeforces Round 897 (Div. 2)","url":"https://codeforces.com/contest/1867/problem/D","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n5 3\n2 3 5 3 4\n4 2\n2 4 3 1\n1 1\n1\n3 1\n1 2 3\n5 3\n5 4 3 2 1\n6 1\n1 2 3 1 5 6\n","output":"YES\nNO\nYES\nYES\nNO\nNO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DCyclicOperations"}}}

use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge::Edge;
use algo_lib::graph::graph::Graph;
use algo_lib::graph::topological_sort::TopologicalSort;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let (n, k) = (input.read_size(), input.read_size());
    let b = input.read_size_vec(n).dec();

    if k == 1 {
        let mut should_be = 0;
        for i in 0..n {
            if b[i] != should_be {
                out.print_line("NO");
                return;
            }
            should_be += 1;
        }
        out.print_line("YES");
        return;
    }

    let mut graph = Graph::new(n);

    let mut up = vec![vec![0; 32]; n];
    for node in 0..n {
        up[node][0] = b[node];
    }

    for exponent in 1..32 {
        for node in 0..n {
            let mid = up[node][exponent - 1];
            up[node][exponent] = up[mid][exponent - 1];
        }
    }

    let next = |vertex: usize, levels: usize| -> usize {
        let mut current = vertex;
        for e in (0..32).rev() {
            if (levels >> e) & 1 == 1 {
                current = up[current][e];
            }
        }
        current
    };

    for start in 0..n {
        let nxt = next(start, k - 1);

        if b[nxt] != start {
            graph.add_edge(nxt, Edge::new(b[nxt]));
        }
    }

    if let Some(_) = graph.topological_sort() {
        out.print_line("YES");
    } else {
        out.print_line("NO");
    }
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
