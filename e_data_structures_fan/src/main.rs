//{"name":"E. Data Structures Fan","group":"Codeforces - Codeforces Round 895 (Div. 3)","url":"https://codeforces.com/contest/1872/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n5\n1 2 3 4 5\n01000\n7\n2 0\n2 1\n1 2 4\n2 0\n2 1\n1 1 3\n2 1\n6\n12 12 14 14 5 5\n001001\n3\n2 1\n1 2 4\n2 1\n4\n7 7 7 777\n1111\n3\n2 0\n1 2 3\n2 0\n2\n1000000000 996179179\n11\n1\n2 1\n5\n1 42 20 47 7\n00011\n5\n1 3 4\n1 1 1\n1 3 4\n1 2 4\n2 0\n","output":"3 2 6 7 7\n11 7\n0 0\n16430827\n47\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EDataStructuresFan"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_int();
    let values = input.read_int_vec(n as usize);
    let string = input.read_string();

    let mut xor_zero = 0;
    let mut xor_one = 0;
    for (i, b) in string.as_bytes().iter().enumerate() {
        if *b == b'1' {
            xor_one ^= values[i];
        } else {
            xor_zero ^= values[i];
        }
    }

    let mut pref_xor = vec![0; (n + 1) as usize];
    for i in 1..=n {
        let i = i as usize;
        pref_xor[i] = pref_xor[i - 1] ^ values[i - 1];
    }

    let q = input.read_int();
    for _ in 0..q {
        let t = input.read_int();
        if t == 1 {
            let (l, r) = (input.read_size(), input.read_size());
            let this_xor = pref_xor[r] ^ pref_xor[l - 1];
            xor_zero ^= this_xor;
            xor_one ^= this_xor;
        } else {
            let b = input.read_int();
            if b == 1 {
                out_line!(xor_one);
            } else {
                out_line!(xor_zero);
            }
        }
    }
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
