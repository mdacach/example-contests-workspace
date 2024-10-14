//{"name":"B. 250 Thousand Tons of TNT","group":"Codeforces - Codeforces Round 909 (Div. 3)","url":"https://codeforces.com/contest/1899/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n2\n1 2\n6\n10 2 3 6 1 3\n4\n1000000000 1000000000 1000000000 1000000000\n15\n60978 82265 78961 56708 39846 31071 4913 4769 29092 91348 64119 72421 98405 222 14294\n8\n19957 69913 37531 96991 57838 21008 14207 19198\n","output":"1\n9\n0\n189114\n112141\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"B250ThousandTonsOfTNT"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let weights = input.read_size_vec(n);

    let mut answer = 0;
    for k in 1..=n {
        if n % k != 0 {
            continue;
        }

        let mut i = 0;
        let mut max = usize::MIN;
        let mut min = usize::MAX;
        loop {
            let mut current_sum = 0;
            for j in 0..k {
                current_sum += weights[i];
                i += 1;
            }
            max.maxim(current_sum);
            min.minim(current_sum);

            if i == n {
                break;
            }
        }

        answer.maxim(max - min);
    }
    out.print_line(answer);
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
