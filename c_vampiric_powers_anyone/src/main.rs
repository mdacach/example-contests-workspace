//{"name":"C. Vampiric Powers, anyone?","group":"Codeforces - Codeforces Round 882 (Div. 2)","url":"https://codeforces.com/contest/1847/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n4\n0 2 5 1\n3\n1 2 3\n5\n8 2 4 12 1\n","output":"7\n3\n14\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CVampiricPowersAnyone"}}}

use std::collections::{BTreeMap, BTreeSet};

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);

    let mut has_subarray_with_xor = |target_xor: usize| {
        let mut seen_xor = BTreeSet::new();
        let mut current_xor = 0;
        seen_xor.insert(current_xor);

        for i in 0..n {
            current_xor ^= a[i];

            let target = current_xor ^ target_xor;

            if seen_xor.contains(&target) {
                return true;
            }

            seen_xor.insert(current_xor);
        }

        return false;
    };

    for target in (0..1 << 8).rev() {
        if has_subarray_with_xor(target) {
            out.print_line(target);
            return;
        }
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
