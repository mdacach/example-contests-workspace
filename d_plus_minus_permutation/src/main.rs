//{"name":"D. Plus Minus Permutation","group":"Codeforces - Codeforces Round 895 (Div. 3)","url":"https://codeforces.com/contest/1872/problem/D","interactive":false,"timeLimit":1000,"tests":[{"input":"8\n7 2 3\n12 6 3\n9 1 9\n2 2 2\n100 20 50\n24 4 6\n1000000000 5575 25450\n4 4 1\n","output":"12\n-3\n44\n0\n393\n87\n179179179436104\n-6\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DPlusMinusPermutation"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

fn sum(r: i64) -> i64 {
    r * (r + 1) / 2
}

fn solve(input: &mut Input, _test_case: usize) {
    let (n, x, y) = (input.read_long(), input.read_long(), input.read_long());
    let good = n / x;
    let bad = n / y;
    let common = n / (lcm(x as usize, y as usize)) as i64;

    let from_good = sum(n) - sum(n - (good - common));
    let from_bad = sum(bad - common);
    out_line!(from_good - from_bad);
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
    }
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
