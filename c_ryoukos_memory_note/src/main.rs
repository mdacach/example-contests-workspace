//{"name":"C. Ryouko's Memory Note","group":"Codeforces - Codeforces Round 248 (Div. 2)","url":"https://codeforces.com/problemset/problem/433/C","interactive":false,"timeLimit":1000,"tests":[{"input":"4 6\n1 2 3 4 3 2\n","output":"3\n"},{"input":"10 5\n9 4 3 8 8\n","output":"6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CRyoukosMemoryNote"}}}

use std::collections::BTreeMap;

use algo_lib::collections::iter_ext::IterExt;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let (n, m) = (input.read_size(), input.read_size());
    let a = input.read_size_vec(m);

    if m == 1 {
        out_line!(0);
        return;
    }

    let mut neighbors: BTreeMap<usize, Vec<usize>> = BTreeMap::new();

    let mut original_diff = 0;
    a.windows(2).for_each(|v| {
        let a = v[0];
        let b = v[1];
        if a == b {
            return;
        }
        let neighbors_a = neighbors.entry(a).or_default();
        neighbors_a.push(b);

        let neighbors_b = neighbors.entry(b).or_default();
        neighbors_b.push(a);

        original_diff += a.abs_diff(b);
    });

    let mut best_improvement = 0;
    for (x, neighbors) in neighbors.iter_mut() {
        neighbors.sort();
        let median = neighbors[neighbors.len() / 2];
        let mut original_diff = 0;
        let mut new_diff = 0;
        for other in neighbors {
            original_diff += x.abs_diff(*other);
            new_diff += median.abs_diff(*other);
        }
        let improvement = original_diff - new_diff;
        if improvement > best_improvement {
            best_improvement = improvement;
        }
    }

    assert!(best_improvement >= 0);
    out_line!(original_diff - best_improvement);
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
