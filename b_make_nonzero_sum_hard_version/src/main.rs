//{"name":"B. Make Nonzero Sum (hard version)","group":"Codeforces - [1400-1500-1600]","url":"https://codeforces.com/group/QsHQUUet4f/contest/488591/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n4\n0 0 0 0\n7\n-1 1 0 1 0 1 0\n5\n0 -1 1 0 1\n3\n1 0 1\n1\n1\n","output":"4\n1 1\n2 2\n3 3\n4 4\n4\n1 1\n2 2\n3 5\n6 7\n-1\n2\n1 1\n2 3\n-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BMakeNonzeroSumHardVersion"}}}

use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::collections::slice_ext::consecutive_iter::ConsecutiveIter;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let values = input.read_int_vec(n);

    let sum = values.iter().sum::<i32>();

    if sum == 0 {
        // One partition for each element.
        out.print_line(n);
        for i in 0..n {
            out.print_line(format!("{} {}", i + 1, i + 1));
        }
    } else {
        if sum % 2 != 0 {
            out.print_line(-1);
            return;
        }

        if sum > 0 {
            // Need to swap some ones
            let mut need_to_swap = sum / 2;
            let mut swapped_last = false;
            let mut start_partition = vec![false; n];
            let mut together = 0;
            for i in 1..n {
                if values[i] != 1 {
                    swapped_last = false;
                    continue;
                }

                if !swapped_last && need_to_swap > 0 {
                    swapped_last = true;
                    need_to_swap -= 1;
                    start_partition[i - 1] = true;
                    together += 1;
                } else {
                    swapped_last = false;
                }
            }
            if need_to_swap > 0 {
                out.print_line(-1);
            } else {
                out.print_line(n - together);
                let mut i = 0;
                while i < n {
                    if start_partition[i] {
                        out.print_line(format!("{} {}", i + 1, i + 2));
                        i += 2;
                    } else {
                        out.print_line(format!("{} {}", i + 1, i + 1));
                        i += 1;
                    }
                }
            }
        } else {
            // Need to swap some minus ones
            let mut need_to_swap = sum.abs() / 2;
            let mut swapped_last = false;
            let mut start_partition = vec![false; n];
            let mut together = 0;
            for i in 1..n {
                if values[i] != -1 {
                    swapped_last = false;
                    continue;
                }

                if !swapped_last && need_to_swap > 0 {
                    swapped_last = true;
                    need_to_swap -= 1;
                    start_partition[i - 1] = true;
                    together += 1;
                } else {
                    swapped_last = false;
                }
            }
            if need_to_swap > 0 {
                out.print_line(-1);
            } else {
                out.print_line(n - together);
                let mut i = 0;
                while i < n {
                    if start_partition[i] {
                        out.print_line(format!("{} {}", i + 1, i + 2));
                        i += 2;
                    } else {
                        out.print_line(format!("{} {}", i + 1, i + 1));
                        i += 1;
                    }
                }
            }
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
