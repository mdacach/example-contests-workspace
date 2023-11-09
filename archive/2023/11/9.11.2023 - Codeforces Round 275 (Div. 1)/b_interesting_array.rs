//{"name":"B. Interesting Array","group":"Codeforces - Codeforces Round 275 (Div. 1)","url":"https://codeforces.com/problemset/problem/482/B","interactive":false,"timeLimit":1000,"tests":[{"input":"3 1\n1 3 3\n","output":"YES\n3 3 3\n"},{"input":"3 2\n1 3 3\n1 3 2\n","output":"NO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BInterestingArray"}}}

use algo_lib::collections::array2d::Array2d;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let (n, m) = (input.read_size(), input.read_size());
    let mut array = vec![0; n + 1];
    let mut delta_encoding = vec![[0; 31]; n + 10];
    let mut queries = vec![];
    for _ in 0..m {
        let (l, r) = (input.read_size(), input.read_size());
        let q = input.read_size();
        queries.push((l, r, q));
        for b in 0..31 {
            if q >> b & 1 == 1 {
                delta_encoding[l][b] += 1;
                delta_encoding[r + 1][b] -= 1;
            }
        }
    }

    let mut bit_count = vec![0; 31];
    for i in 1..=n {
        let mut number = 0;
        for b in 0..31 {
            bit_count[b] += delta_encoding[i][b];
        }

        for b in 0..31 {
            if bit_count[b] >= 1 {
                number += 1 << b;
            }
        }

        array[i] = number;
    }

    // let mut table = Array2d::new(31, n + 1, 0);
    let mut table = vec![vec![0; n + 1]; 31];
    for i in 1..=n {
        table[0][i] = array[i];
    }
    for b in 1..31 {
        for i in 1..=n {
            if i + (1 << (b - 1)) > n {
                continue;
            }
            table[b][i] = table[b - 1][i] & table[b - 1][i + (1 << (b - 1))];
        }
    }

    for (l, r, q) in queries {
        let mut answer = (1 << 31) - 1;
        let diff = r - l + 1;
        let mut next_index = l;
        for b in 0..31 {
            if diff >> b & 1 == 1 {
                answer = answer & table[b][next_index];
                next_index += (1 << b);
            }
        }

        if answer != q {
            out_line!("NO");
            return;
        }
    }

    out_line!("YES");
    for i in 1..=n {
        out!(array[i], " ");
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
