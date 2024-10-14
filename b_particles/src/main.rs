//{"name":"B. Particles","group":"Codeforces - [1200, 1300, 1500, 1600]","url":"https://codeforces.com/group/QsHQUUet4f/contest/489749/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n6\n-3 1 4 -1 5 -9\n5\n998244353 998244353 998244353 998244353 998244353\n1\n-2718\n","output":"9\n2994733059\n-2718\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BParticles"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let values = input.read_long_vec(n);

    #[cfg(feature = "test")]
    out.print_line("hello test");

    let mut dp = vec![0; n + 1];
    let mut x = 5;

    // for y in &values {
    //     y = 2;
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
