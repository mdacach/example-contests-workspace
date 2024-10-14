//{"name":"C. Serval and Toxel's Arrays","group":"Codeforces - Codeforces Round 853 (Div. 2)","url":"https://codeforces.com/contest/1789/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n3 2\n1 2 3\n1 4\n2 5\n1 1\n1\n1 1\n10 10\n4 6 9 12 16 20 2 10 19 7\n1 3\n5 4\n2 17\n2 18\n6 11\n7 1\n8 17\n5 5\n5 5\n2 2\n","output":"13\n1\n705\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CServalAndToxelsArrays"}}}

use algo_lib::collections::segment_tree::SegmentTree;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let (n, m) = (input.read_size(), input.read_size());
    let mut values = input.read_size_vec(n);

    let mut count = vec![0; 200_005];
    for &v in &values {
        count[v] += 1;
    }
    let mut sum_of_occurrences = n as i64;
    let mut answer = 0;
    for operation in 1..=m {
        let previous_arrays = operation as i64;

        let index = input.read_size() - 1;
        let new_value = input.read_size();

        let old_value = values[index];
        values[index] = new_value;

        answer += previous_arrays * n as i64;

        answer += previous_arrays * n as i64;

        sum_of_occurrences += count[new_value];
        sum_of_occurrences -= count[old_value];

        answer -= sum_of_occurrences;

        for &v in &values {
            count[v] += 1;
        }

        sum_of_occurrences += n as i64;
    }

    out.print_line(answer);
}

fn _solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let (n, m) = (input.read_size(), input.read_size());
    let mut values = input.read_size_vec(n);

    let mut count = vec![0; 200_005];
    for &v in &values {
        count[v] += 1;
    }
    let mut sum_of_occurrences = 0;
    let mut answer = 0;
    for operation in 1..=m {
        let previous_arrays = operation as i64;

        let index = input.read_size() - 1;
        let new_value = input.read_size();

        let old_value = values[index];
        values[index] = new_value;

        answer += previous_arrays * n as i64;
        // answer += previous_arrays * n as i64 - sum_of_occurrences - n as i64;

        answer += previous_arrays * n as i64;

        sum_of_occurrences = 0;
        for &v in &values {
            sum_of_occurrences += count[v];
            count[v] += 1;
        }
        answer -= sum_of_occurrences;
        // sum_of_occurrences += count[new_value];
        // sum_of_occurrences -= count[old_value];
        //
        for &v in &values {
            // answer -= count[v] as i64;
            // answer += (previous_arrays - count[v] as i64);
        }

        for &v in &values {
            count[v] += 1;
        }

        sum_of_occurrences += n as i64;
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
