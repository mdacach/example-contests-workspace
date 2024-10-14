//{"name":"D1. Maximum And Queries (easy version)","group":"Codeforces - Codeforces Round 912 (Div. 2)","url":"https://codeforces.com/contest/1903/problem/D1","interactive":false,"timeLimit":2000,"tests":[{"input":"4 2\n1 3 7 5\n2\n10\n","output":"2\n6\n"},{"input":"3 5\n4 0 2\n9\n8\n17\n1\n3\n","output":"5\n4\n7\n0\n1\n"},{"input":"1 2\n10\n5\n2318381298321\n","output":"15\n2318381298331\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"D1MaximumAndQueriesEasyVersion"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let (n, q) = (input.read_size(), input.read_size());
    let values = (input.read_u64_vec(n));

    for _ in 0..q {
        let k = input.read_u64();

        let mut values = values.clone();

        let mut maximum_and = 0_u64;
        let mut operations_remaining = k;
        for bit_i_want_to_set in (0..64).rev() {
            // For this bit to be set in the AND, it must be set in each value.
            // Because this bit is always better than all the smaller summed (2^K > 2^0 + 2^1 + ... + 2^K-1), we can greedily try it
            // first.
            let mut operations_needed_for_this_bit = 0_u64;
            for &value in &values {
                let this_bit_is_set = (value >> bit_i_want_to_set) & 1 == 1;
                if this_bit_is_set {
                    continue;
                }

                let current_value = if bit_i_want_to_set == 63 {
                    value
                } else {
                    let mask_to_unset_used_bits = (1 << (bit_i_want_to_set + 1)) - 1;
                    value & mask_to_unset_used_bits
                };

                let want_to_reach = 1 << bit_i_want_to_set;
                let operations_needed_here = want_to_reach - current_value;

                operations_needed_for_this_bit =
                    operations_needed_for_this_bit.saturating_add(operations_needed_here);
            }

            if operations_needed_for_this_bit <= operations_remaining {
                operations_remaining -= operations_needed_for_this_bit;
                maximum_and += 1 << bit_i_want_to_set;

                for value in &mut values {
                    let this_bit_is_set = (*value >> bit_i_want_to_set) & 1 == 1;
                    if this_bit_is_set {
                        continue;
                    }

                    // Update the original value
                    let current_value = if bit_i_want_to_set == 63 {
                        *value
                    } else {
                        let mask_to_unset_used_bits = (1 << (bit_i_want_to_set + 1)) - 1;
                        *value & mask_to_unset_used_bits
                    };
                    let want_to_reach = 1 << bit_i_want_to_set;
                    let operations_needed_here = want_to_reach - current_value;
                    *value += operations_needed_here;
                }
            }
        }

        out.print_line(maximum_and);
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
