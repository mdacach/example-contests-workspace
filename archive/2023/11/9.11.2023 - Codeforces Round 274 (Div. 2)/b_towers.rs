//{"name":"B. Towers","group":"Codeforces - Codeforces Round 274 (Div. 2)","url":"https://codeforces.com/problemset/problem/479/B","interactive":false,"timeLimit":1000,"tests":[{"input":"3 2\n5 8 5\n","output":"0 2\n2 1\n2 3\n"},{"input":"3 4\n2 2 4\n","output":"1 1\n3 2\n"},{"input":"5 3\n8 3 2 6 3\n","output":"3 3\n1 3\n1 2\n1 3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BTowers"}}}

use std::collections::{BTreeMap, BTreeSet};

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let (n, k) = (input.read_size(), input.read_size());
    let mut towers: BTreeSet<(usize, usize)> = (0..n)
        .map(|_| input.read_size())
        .enumerate()
        .map(|(i, v)| (v, i))
        .collect();

    let mut operations = vec![];
    for _ in 0..k {
        let biggest = towers.last().unwrap().clone();
        let smallest = towers.first().unwrap().clone();
        if biggest.0 == smallest.0 {
            break;
        }
        operations.push((biggest.1, smallest.1));
        towers.remove(&biggest);
        towers.remove(&smallest);

        let new_from_biggest = (biggest.0 - 1, biggest.1);
        let new_from_smallest = (smallest.0 + 1, smallest.1);
        towers.insert(new_from_biggest);
        towers.insert(new_from_smallest);
    }
    let biggest = towers.last().unwrap().clone();
    let smallest = towers.first().unwrap().clone();
    let difference = biggest.0 - smallest.0;

    out_line!(difference, operations.len());
    for operation in operations {
        out_line!(operation.0 + 1, operation.1 + 1);
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
