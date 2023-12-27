//{"name":"A. Sets and Union","group":"Codeforces - [1300, 1400, 1500, 1600]","url":"https://codeforces.com/group/QsHQUUet4f/contest/488797/problem/A","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n3\n3 1 2 3\n2 4 5\n2 3 4\n4\n4 1 2 3 4\n3 2 5 6\n3 3 5 6\n3 4 5 6\n5\n1 1\n3 3 6 10\n1 9\n2 1 3\n3 5 8 9\n1\n2 4 28\n","output":"4\n5\n6\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ASetsAndUnion"}}}

use std::collections::BTreeSet;

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

// The resulting set S must be different than the union of all the sets.
// This means it must be missing one or more elements.
//
// We are going to test all the best possible sets:
//  Let's iterate over the missing element x.
//  Because we want to maximize the size of S, we will want to union it with all the available sets,
//  minus the ones that have x.
//  In the end, the set will
//  1. be as big as possible (because we are unionizing it with all the
//  allowed sets) and,
//  2. be smaller than the union (because it's missing, at least, x).
//
// Note that by iterating over the missing element x, we exhaust all possible _solutions_.
// Consider this example:
//  Union: {1, 2, 3, 4}
//  Sets: {1}, {1, 2}, {1, 3}, {1, 4}
//
//  A possible construction is the set {1}, but our solution will never reach that (because no
//  matter which missing element you will be considering, the chosen set will always also contain
//  2, 3, or 4).
//  This is not a problem because {1} will never be a solution. It would require us to consider
//  excluding all of {2, 3, 4} at once, which is _too_ restrictive.
//
//  So our solution considers all of the **solution** space, but not the whole space. Which is OK.
fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let mut sets = vec![vec![]; n];
    let mut elements_in_union = BTreeSet::new();
    for i in 0..n {
        let s = input.read_size();
        let v = input.read_size_vec(s);
        for &v in &v {
            elements_in_union.insert(v);
        }
        sets[i] = v;
    }
    let elements_in_union = elements_in_union.len();

    let mut answer = 0;
    for removed_element in 1..=50 {
        let mut count = vec![0; 55];
        for s in &sets {
            if s.contains(&removed_element) {
                // skip this set
            } else {
                for &v in s {
                    count[v] += 1;
                }
            }
        }
        let here = count.into_iter().filter(|x| *x >= 1).count();
        if here != elements_in_union {
            answer.maxim(here);
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
