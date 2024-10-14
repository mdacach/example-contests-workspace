//{"name":"D. Professor Higashikata","group":"Codeforces - Codeforces Round 882 (Div. 2)","url":"https://codeforces.com/contest/1847/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"2 2 4\n01\n1 2\n1 2\n1\n1\n2\n2\n","output":"0\n1\n0\n1\n"},{"input":"8 6 10\n10011010\n5 6\n2 3\n6 8\n5 7\n5 8\n6 8\n3\n5\n6\n2\n5\n2\n5\n8\n4\n1\n","output":"2\n3\n2\n2\n1\n2\n2\n2\n2\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DProfessorHigashikata"}}}

use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::collections::treap_map::TreapSet;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::StrReader;
use std::cmp::min;
use std::collections::{BTreeMap, BTreeSet, HashMap};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let (n, m, q) = (input.read_size(), input.read_size(), input.read_size());
    let mut binary_string = input.read_str();

    let mut opening = vec![vec![]; n + 5];
    let mut closing = vec![vec![]; n + 5];
    for segment_id in 0..m {
        let (l, r) = (input.read_size() - 1, input.read_size() - 1);
        opening[l].push(segment_id);
        closing[r + 1].push(segment_id);
    }

    let mut first_occurrence = vec![None; n];
    let mut active = BTreeSet::new();
    for i in 0..n {
        for id in &closing[i] {
            active.remove(id);
        }
        for &id in &opening[i] {
            active.insert(id);
        }

        first_occurrence[i] = active.first().copied();
    }

    let mut ids = (0..n)
        .filter(|&i| first_occurrence[i].is_some())
        .collect_vec();
    ids.sort_by(|a, b| first_occurrence[*a].cmp(&first_occurrence[*b]));
    let mut id_to_position = BTreeMap::new();
    for (position, id) in ids.iter().enumerate() {
        id_to_position.insert(*id, position);
    }

    let ones = ids
        .iter()
        .filter(|&&i| binary_string[i] == b'1')
        .copied()
        .collect_vec();

    let mut one_positions = TreapSet::new();
    for id in &ones {
        one_positions.insert(id_to_position[id]);
    }

    let can_fit_at_most = ids.len();
    let mut count_ones = binary_string.iter().filter(|&x| x == b'1').count();
    for _query in 0..q {
        let swapped = input.read_size() - 1;
        if binary_string[swapped] == b'1' {
            count_ones -= 1;
            binary_string[swapped] = b'0';
            if let Some(position) = id_to_position.get(&swapped) {
                one_positions.remove(position);
            }
        } else {
            count_ones += 1;
            binary_string[swapped] = b'1';
            if let Some(position) = id_to_position.get(&swapped) {
                one_positions.insert(*position);
            }
        }

        let can_make = min(count_ones, can_fit_at_most);
        let already_right = one_positions.less(&can_make);
        let operations_needed = can_make - already_right;
        out.print_line(operations_needed);
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
