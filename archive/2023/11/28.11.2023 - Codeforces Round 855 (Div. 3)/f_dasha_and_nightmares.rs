//{"name":"F. Dasha and Nightmares","group":"Codeforces - Codeforces Round 855 (Div. 3)","url":"https://codeforces.com/contest/1800/problem/F","interactive":false,"timeLimit":4000,"tests":[{"input":"10\nftl\nabcdefghijklmnopqrstuvwxy\nabcdeffghijkllmnopqrsttuvwxy\nffftl\naabbccddeeffgghhiijjkkllmmnnooppqqrrssttuuvvwwxxyy\nthedevid\nbcdefghhiiiijklmnopqrsuwxyz\ngorillasilverback\nabcdefg\nijklmnopqrstuvwxyz\n","output":"5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FDashaAndNightmares"}}}

use std::collections::HashMap;

use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::string::hash::{HashBase, Hashable, SimpleHash, StringHash};
use algo_lib::string::str::{Str, StrReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    HashBase::init();
    let n = input.read_size();
    let words = input.read_str_vec(n);

    let letter_occurrences = |w: &Str| {
        let mut occurrences = vec![0; 26];
        for c in w.iter() {
            occurrences[(c - b'a') as usize] += 1;
        }
        occurrences
    };

    let mask_from_occurrences = |occurrences: &[usize]| {
        let mut mask: u32 = 0;
        for i in 0..26 {
            if occurrences[i] % 2 != 0 {
                mask.set_bit(i);
            }
        }
        mask
    };

    let mask_from_string = |w: &Str| {
        let occurrences = letter_occurrences(&w);
        mask_from_occurrences(&occurrences)
    };

    const FULL_MASK: u32 = (1 << 26) - 1;
    let mut answer: u64 = 0;
    for missing_letter in b'a'..=b'z' {
        let masks = words
            .iter()
            .filter(|w| !w.contains(&missing_letter))
            .map(|w| mask_from_string(w))
            .collect_vec();

        let mut seen_masks = HashMap::new();
        let letter_index = (missing_letter - b'a') as usize;
        for &m in &masks {
            *seen_masks.entry(m).or_default() += 1;

            let mask_without_ignored_letter = FULL_MASK ^ (1 << letter_index);
            let target_mask = mask_without_ignored_letter ^ m;

            answer += seen_masks.get(&target_mask).unwrap_or(&0);
        }
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
