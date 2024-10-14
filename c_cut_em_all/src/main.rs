//{"name":"C. Cut 'em all!","group":"Codeforces - Codeforces Round 484 (Div. 2)","url":"https://codeforces.com/problemset/problem/982/C","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n2 4\n4 1\n3 1\n","output":"1\n"},{"input":"3\n1 2\n1 3\n","output":"-1\n"},{"input":"10\n7 1\n8 4\n8 10\n4 7\n6 5\n9 3\n3 5\n2 10\n2 5\n","output":"4\n"},{"input":"2\n1 2\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CCutEmAll"}}}

use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let mut edges = vec![];
    for _ in 1..n {
        let (a, b) = (input.read_size() - 1, input.read_size() - 1);
        edges.push((a, b));
    }
    let tree = Graph::from_biedges(n, &edges);
    let mut cut = 0;

    let mut impossible = false;
    let mut solve_for_subtree = RecursiveFunction2::new(|this_function, current, parent| {
        let mut odd_children = 0;
        for child in &tree[current] {
            let to = child.to();
            if to == parent {
                continue;
            }

            let parity = this_function.call(to, current);
            if parity == 0 {
                cut += 1;
            } else {
                odd_children += 1;
            }
        }

        if odd_children % 2 == 0 {
            if current == 0 {
                impossible = true;
            }
            1
        } else {
            0
        }
    });
    solve_for_subtree.call(0, 0);

    if impossible {
        out.print_line(-1);
    } else {
        out.print_line(cut);
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
