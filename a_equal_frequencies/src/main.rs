//{"name":"A. Equal Frequencies","group":"Codeforces - [1600 x4]","url":"https://codeforces.com/group/QsHQUUet4f/contest/489465/problem/A","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n5\nhello\n10\ncodeforces\n5\neevee\n6\nappall\n","output":"1\nhelno\n2\ncodefofced\n1\neeeee\n0\nappall\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AEqualFrequencies"}}}

use std::collections::BTreeMap;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::numbers::primes::factorize::Factorize;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let s = input.read_str();

    let mut frequency_map: BTreeMap<u8, i64> = BTreeMap::new();
    for c in s {
        *frequency_map.entry(c).or_default() += 1;
    }
    let mut sorted_by_frequency = vec![];
    for (c, _) in &frequency_map {
        sorted_by_frequency.push(c);
    }
    sorted_by_frequency.sort_unstable_by(|a, b| frequency_map[b].cmp(&frequency_map[a]));

    for d in n.divisors() {
        let different_letters = d;
        let frequency_needed = n as i64 / d;

        let mut extra: Vec<usize> = vec![];
        let mut current_letter = *frequency_map.first_entry().unwrap().key();
        let mut last_letter_index = sorted_by_frequency.len() - 1;
        for (letter, &count) in &frequency_map {
            if count + extra.len() as i64 >= frequency_needed {
                // Change some from extra, potentially leaving some.
            } else {
                // We need to get from the right too
                let last_letter = sorted_by_frequency[last_letter_index];
                let last_count = frequency_map[last_letter];
                frequency_map[last_letter] += 2;
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
