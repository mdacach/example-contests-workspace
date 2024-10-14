//{"name":"G. Unusual Entertainment","group":"Codeforces - Codeforces Round 909 (Div. 3)","url":"https://codeforces.com/contest/1899/problem/G","interactive":false,"timeLimit":3000,"tests":[{"input":"3\n3 5\n1 2\n2 3\n1 2 3\n1 2 2\n1 2 3\n2 3 1\n1 2 3\n2 3 3\n10 10\n2 6\n2 7\n2 4\n1 7\n2 8\n10 6\n8 5\n9 4\n3 4\n10 2 5 9 1 7 6 4 3 8\n8 9 8\n7 8 1\n7 10 6\n4 8 9\n5 5 10\n7 10 1\n9 9 2\n9 10 6\n6 6 2\n10 10 6\n1 1\n1\n1 1 1\n","output":"YES\nNO\nYES\nNO\nYES\n\nNO\nYES\nYES\nYES\nNO\nYES\nYES\nNO\nNO\nNO\n\nYES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GUnusualEntertainment"}}}

use std::collections::{BTreeSet, HashMap};

use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge_trait::{self, EdgeTrait};
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let (n, q) = (input.read_size(), input.read_size());
    let edges = input.read_size_pair_vec(n - 1).dec();
    let tree = Graph::from_biedges(n, &edges);

    let order = input.read_size_vec(n).dec();
    let mut position_in_order = vec![0; n];
    for (i, &pos) in order.iter().enumerate() {
        position_in_order[pos] = i;
    }

    let mut answer_for_queries = vec![false; q];
    let mut queries_with_target = HashMap::new();
    for id in 0..q {
        let (l, r, v) = (
            input.read_size() - 1,
            input.read_size() - 1,
            input.read_size() - 1,
        );
        queries_with_target
            .entry(v)
            .or_insert(vec![])
            .push((l, r, id));
    }

    let mut subtree_size = vec![1; n];
    let mut compute_subtree_size = RecursiveFunction2::new(|f, current, parent| {
        for child in &tree[current] {
            let child = child.to();
            if child == parent {
                continue;
            }

            f.call(child, current);
        }

        for child in &tree[current] {
            let child = child.to();
            if child == parent {
                continue;
            }
            subtree_size[current] += subtree_size[child];
        }
    });
    compute_subtree_size.call(0, 0);

    let mut dfs = RecursiveFunction2::new(|f, current: usize, parent: usize| -> BTreeSet<usize> {
        let mut children = tree[current]
            .iter()
            .map(edge_trait::EdgeTrait::to)
            .filter(|x| *x != parent)
            .collect_vec();
        children.sort_unstable_by(|a, b| subtree_size[*b].cmp(&subtree_size[*a]));

        let mut all_active = BTreeSet::new();
        let mut first = true;
        for child in children {
            if child == parent {
                continue;
            }

            let mut active = f.call(child, current);
            if first {
                all_active = active;
                first = false;
            } else {
                // all_active.append(&mut active);
                all_active.extend(active);
            }
        }

        all_active.insert(position_in_order[current]);

        if let Some(queries) = queries_with_target.get(&current) {
            for &(l, r, id) in queries {
                if let Some(&_v) = all_active.range(l..=r).next() {
                    answer_for_queries[id] = true;
                }
            }
        };

        all_active
    });
    dfs.call(0, 0);

    for answer in answer_for_queries {
        out.print_line(if answer { "YES" } else { "NO" });
    }
    out.print_line("");
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
