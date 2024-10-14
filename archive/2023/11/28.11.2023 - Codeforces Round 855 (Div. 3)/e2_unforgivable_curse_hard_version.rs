//{"name":"E2. Unforgivable Curse (hard version)","group":"Codeforces - Codeforces Round 855 (Div. 3)","url":"https://codeforces.com/contest/1800/problem/E2","interactive":false,"timeLimit":1000,"tests":[{"input":"7\n6 3\ntalant\natltna\n7 1\nabacaba\naaaabbc\n12 6\nabracadabraa\navadakedavra\n5 3\naccio\ncicao\n5 4\nlumos\nmolus\n4 3\nuwjt\ntwju\n4 3\nkvpx\nvxpk\n","output":"YES\nYES\nNO\nYES\nNO\nYES\nNO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"E2UnforgivableCurseHardVersion"}}}

use std::collections::BTreeMap;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::StrReader;

type PreCalc = ();

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
