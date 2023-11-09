//{"name":"C. Little Girl and Maximum Sum","group":"Codeforces - Codeforces Round 169 (Div. 2)","url":"https://codeforces.com/problemset/problem/276/C","interactive":false,"timeLimit":1000,"tests":[{"input":"3 3\n5 3 2\n1 2\n2 3\n1 3\n","output":"25\n"},{"input":"5 3\n5 2 4 1 3\n1 5\n2 3\n2 3\n","output":"33\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CLittleGirlAndMaximumSum"}}}

use std::collections::BinaryHeap;

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let (n, q) = (input.read_size(), input.read_size());
    let mut values = input.read_size_vec(n);

    let mut frequency = vec![0_i32; n + 1];
    for _ in 0..q {
        let (start, end) = (input.read_size(), input.read_size());
        frequency[start - 1] += 1;
        frequency[end] -= 1;
    }
    for i in 1..n {
        frequency[i] += frequency[i - 1];
    }

    values.sort_unstable_by(|a, b| b.cmp(a));
    let mut indexes = Vec::from_iter(0..n);
    indexes.sort_unstable_by(|&a, &b| frequency[b].cmp(&frequency[a]));

    let mut answer_array = vec![0; n];
    indexes
        .into_iter()
        .zip(values.into_iter())
        .for_each(|(index, value)| answer_array[index] = value);

    let mut total_sum = 0;
    for i in 0..n {
        total_sum += answer_array[i] * frequency[i] as usize;
    }
    out_line!(total_sum);
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
