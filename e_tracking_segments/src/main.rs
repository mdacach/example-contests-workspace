//{"name":"E. Tracking Segments","group":"Codeforces - Codeforces Round 881 (Div. 3)","url":"https://codeforces.com/problemset/problem/1843/E","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n5 5\n1 2\n4 5\n1 5\n1 3\n2 4\n5\n5\n3\n1\n2\n4\n4 2\n1 1\n4 4\n2\n2\n3\n5 2\n1 5\n1 5\n4\n2\n1\n3\n4\n5 2\n1 5\n1 3\n5\n4\n1\n2\n3\n5\n5 5\n1 5\n1 5\n1 5\n1 5\n1 4\n3\n1\n4\n3\n3 2\n2 2\n1 3\n3\n2\n3\n1\n","output":"3\n-1\n3\n3\n3\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ETrackingSegments"}}}

use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let (n, m) = (input.read_size(), input.read_size());
    let segments = input.read_size_pair_vec(m).dec();
    let q = input.read_size();
    let changes = input.read_size_vec(q).dec();

    let mut is_ok_at = |c| {
        if c > q {
            return true;
        }

        let mut values = vec![0; n];
        changes.iter().take(c).for_each(|id| values[*id] = 1);

        let mut prefix_sum = vec![0; n];
        prefix_sum[0] = values[0];
        for i in 1..n {
            prefix_sum[i] = prefix_sum[i - 1] + values[i];
        }
        let get_sum = |a, b| {
            if a == 0 {
                prefix_sum[b]
            } else {
                prefix_sum[b] - prefix_sum[a - 1]
            }
        };

        for &(l, r) in &segments {
            let length = r - l + 1;
            if get_sum(l, r) > length / 2 {
                return true;
            }
        }

        return false;
    };

    let mut left = 0; // 0 is always bad, as the array starts at all zeros.
    let mut right = q + 1; // q+1 is specially treated as true;
    while right - left > 1 {
        let middle = left + (right - left) / 2;
        if is_ok_at(middle) {
            right = middle;
        } else {
            left = middle;
        }
    }
    if right > q {
        out.print_line(-1);
    } else {
        out.print_line(right);
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
