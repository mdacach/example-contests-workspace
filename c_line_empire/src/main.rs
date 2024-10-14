//{"name":"C. Line Empire","group":"Codeforces - Codeforces Round 782 (Div. 2)","url":"https://codeforces.com/problemset/problem/1659/C","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n5 2 7\n3 5 12 13 21\n5 6 3\n1 5 6 21 30\n2 9 3\n10 15\n11 27182 31415\n16 18 33 98 874 989 4848 20458 34365 38117 72030\n","output":"173\n171\n75\n3298918744\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CLineEmpire"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use std::collections::BTreeMap;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let (n, a, b) = (input.read_size(), input.read_u64(), input.read_u64());
    let positions = {
        let mut v = vec![0];
        v.append(&mut input.read_u64_vec(n));
        v
    };

    let mut prefix_sum = vec![0; n + 1];
    for i in 1..=n {
        prefix_sum[i] = prefix_sum[i - 1] + positions[i];
    }
    let get_sum = |l, r| {
        // Bad for l == 0, but whatever.
        prefix_sum[r] - prefix_sum[l - 1]
    };

    let mut current_capital = 0;
    let mut current_cost = 0;
    for current_city in 1..=n {
        // Conquer this city.
        current_cost += b * (positions[current_city] - positions[current_capital]);

        if current_city == n {
            // We are done, as there's no use to moving the capital to the last city.
            break;
        }

        // When is it good to move the capitals?
        let remaining_cities = n as u64 - (current_city + 1) as u64;
        let sum_of_other_positions = get_sum(current_city + 1, n);
        let new_price_if_capital_here =
            b * (sum_of_other_positions - (remaining_cities * positions[current_city]));
        let old_price =
            b * (sum_of_other_positions - (remaining_cities * positions[current_capital]));
        let price_of_making_capital = a * (positions[current_city] - positions[current_capital]);

        if new_price_if_capital_here <= old_price + price_of_making_capital {
            current_capital = current_city;
            current_cost += a * (positions[current_city] - positions[current_capital]);
        }
    }

    out.print_line(current_cost);
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
