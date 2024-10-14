//{"name":"C. Factorial Divisibility","group":"Codeforces - [1400-1500-1600]","url":"https://codeforces.com/group/QsHQUUet4f/contest/488591/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"6 4\n3 2 2 2 3 3\n","output":"Yes\n"},{"input":"8 3\n3 2 2 2 2 2 1 1\n","output":"Yes\n"},{"input":"7 8\n7 7 7 7 7 7 7\n","output":"No\n"},{"input":"10 5\n4 3 2 1 4 3 2 4 3 4\n","output":"No\n"},{"input":"2 500000\n499999 499999\n","output":"No\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CFactorialDivisibility"}}}

use algo_lib::collections::default_map::default_tree_map::DefaultTreeMap;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let (n, x) = (input.read_size(), input.read_size());
    let values = input.read_size_vec(n);

    let mut count = vec![0; 500005];
    for v in values {
        count[v] += 1;
    }

    for v in 1..x {
        if count[v] % (v + 1) == 0 {
            count[v + 1] += count[v] / (v + 1);
            count[v] = 0;
        } else {
            out.print_line("No");
            return;
        }
    }
    if count[x] != 0 {
        out.print_line("Yes");
    } else {
        out.print_line("No");
    }

    // for (&v, &mut c) in count.iter_mut() {
    //     let next_entry = v + 1;
    //     if c % (v + 1) == 0 {
    //         count[v + 1] += c / (v + 1);
    //     }
    // }
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
