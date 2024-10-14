//{"name":"E2. Unforgivable Curse (hard version)","group":"Codeforces - Codeforces Round 855 (Div. 3)","url":"https://codeforces.com/contest/1800/problem/E2","interactive":false,"timeLimit":1000,"tests":[{"input":"7\n6 3\ntalant\natltna\n7 1\nabacaba\naaaabbc\n12 6\nabracadabraa\navadakedavra\n5 3\naccio\ncicao\n5 4\nlumos\nmolus\n4 3\nuwjt\ntwju\n4 3\nkvpx\nvxpk\n","output":"YES\nYES\nNO\nYES\nNO\nYES\nNO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"E2UnforgivableCurseHardVersion"}}}

use std::collections::BTreeMap;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::StrReader;

type PreCalc = ();

// 1. Some letters of `s` will never change place.
// We can only swap letters with distance k or k+1.
// If a letter has no other letters with such position, it will never get swapped -> it will never
// change.
// So if any of those letters do not match, we will never be able to construct `t`.
//
// 2. The letter count for `s` and `t` must match.
// No operation will change a letter. So we must have all letters of `t` in `s`, and vice versa.
//
// 3. For the letters that can swap, we can arbitrarily place them.
// One such way of seeing that is creating a construction.
// K = 7
// xxxx aaa xxxx
// ^     ^ will never change
// can be freely moved
//
// For each x, we can always move it to the other side and back.
// xxXx aaa xxxx
//   ^       ^ here
//   we want to get this X
//
// K = 7 to the right
// xxxx aaa xxXx
// K = 8 to the left
// xXxx aaa xxxx
// K = 7 to the right
// xxxx aaa xXxx
//
// By moving between sides, we can arbitrarily move it either 1 step to the left or 1 step to the
// right.
//
// Repeat for each letter.
//
//
//
// My thoughts:
// Actually quite tricky. I've found this problem hard, particularly coupled with the small test
// cases.
// 1. Be more certain before implementing.
// 2. Reassess everything after WA - I kept trying to make my wrong solution work.
// 3. Run a bunch of tests locally, both to verify the solution but also to come up with it.
fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let (n, k) = (input.read_size(), input.read_size());
    let (s, t) = (input.read_str(), input.read_str());

    if s == t {
        out.print_line("YES");
        return;
    }

    let mut letter_count_s: BTreeMap<u8, usize> = BTreeMap::new();
    let mut letter_count_t: BTreeMap<u8, usize> = BTreeMap::new();
    for c in s.clone() {
        *letter_count_s.entry(c).or_default() += 1;
    }
    for c in t.clone() {
        *letter_count_t.entry(c).or_default() += 1;
    }

    for (c, count) in letter_count_s {
        let Some(count_other) = letter_count_t.get(&c) else {
            out.print_line("NO");
            return;
        };
        if *count_other != count {
            out.print_line("NO");
            return;
        }
    }

    for i in 0..n {
        let left = i.abs_diff(0);
        let right = i.abs_diff(n - 1);
        if std::cmp::max(left, right) < k {
            if s[i] != t[i] {
                out.print_line("NO");
                return;
            }
        }
    }
    out.print_line("YES");

    // if n >= k + 3 {
    //     out.print_line("YES");
    // } else if n == k + 2 {
    //     let ok = if n % 2 == 0 {
    //         s[n / 2] == t[n / 2] && s[n / 2 + 1] == t[n / 2 + 1]
    //     } else {
    //         s[n / 2] == t[n / 2]
    //     };
    //     out.print_line(if ok { "YES" } else { "NO" });
    // } else if n == k + 1 {
    //     let first = s.clone();
    //     let mut second = s.clone();
    //     second.swap(0, n - 1);
    //     if first == t || second == t {
    //         out.print_line("YES");
    //     } else {
    //         out.print_line("NO");
    //     }
    // } else {
    //     out.print_line("NO");
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
