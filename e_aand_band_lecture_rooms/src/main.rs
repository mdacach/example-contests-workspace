//{"name":"E. A and B and Lecture Rooms","group":"Codeforces - Codeforces Round 294 (Div. 2)","url":"https://codeforces.com/problemset/problem/519/E","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n1 2\n1 3\n2 4\n1\n2 3\n","output":"1\n"},{"input":"4\n1 2\n2 3\n2 4\n2\n1 2\n1 3\n","output":"0\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EAAndBAndLectureRooms"}}}

use std::collections::VecDeque;

use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{
    Callable3, Callable4, RecursiveFunction3, RecursiveFunction4,
};
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let mut tree = Graph::new(n);
    for _ in 1..n {
        let (a, b) = (input.read_size() - 1, input.read_size() - 1);
        tree.add_edge(a, b);
        tree.add_edge(b, a);
    }

    let root = 0;
    let mut subtree_size = vec![0; n];
    let mut level = vec![0; n];
    let mut dfs = RecursiveFunction3::new(|f, current, parent, _level| {
        level[current] = _level;

        for &adjacent in &tree.adjacency_list[current] {
            if adjacent == parent {
                continue;
            }
            f.call(adjacent, current, _level + 1);
            subtree_size[current] += subtree_size[adjacent];
        }
        subtree_size[current] += 1;
    });
    dfs.call(root, root, 0);

    let lca_table = tree.lowest_common_ancestor_table(root);

    let queries = input.read_size();
    for _ in 0..queries {
        let (mut high, mut low) = (input.read_size() - 1, input.read_size() - 1);
        if high == low {
            // If they are the same, all vertices will have the same distance.
            out_line!(n);
            continue;
        }

        // Let's maintain that `low` is the lowest vertex;
        if level[high] > level[low] {
            std::mem::swap(&mut high, &mut low);
        }

        if lca_table.is_ancestor(high, low) {
            debug_assert!(level[low] > level[high]);

            // `low` must go up, and `high` must go down.
            let total_distance = level[low] - level[high];
            // They must meet at the middle.
            if total_distance % 2 != 0 {
                out_line!(0);
                continue;
            }
            let distance_up = total_distance / 2;

            // We compute the meeting point by climbing up from `low`.
            let meeting_point = lca_table.up(low, distance_up);

            // All _other_ vertices in the subtree of `meeting_point` should work.
            let just_below_meeting = lca_table.up(low, distance_up - 1);
            let other_vertices_in_subtree =
                subtree_size[meeting_point] - subtree_size[just_below_meeting];

            out_line!(other_vertices_in_subtree);
        } else {
            if level[high] == level[low] {
                // They will meet exactly at the LCA.
                let meeting_point = lca_table.of(high, low);

                let distance = level[high] - level[meeting_point];
                let just_below_from_a = lca_table.up(high, distance - 1);
                let just_below_from_b = lca_table.up(low, distance - 1);
                // Because they are meeting at the LCA, there's two possibilities for extra
                // vertices with the same distance.

                // 1. Vertices above the LCA
                let vertices_above = n - subtree_size[meeting_point];
                // 2. Vertices below other than in `low` or `high` paths.
                let vertices_below = subtree_size[meeting_point]
                    - (subtree_size[just_below_from_a] + subtree_size[just_below_from_b]);

                out_line!(vertices_above + vertices_below);
            } else {
                // They will meet below the LCA.
                // `high` must go to the LCA, then down.
                // `low` must go up, but not all the way to the LCA.

                let lca = lca_table.of(high, low);

                // Compute which distance `low` must climb.
                // As they must meet in the middle, both of them must move total_distance / 2;
                let total_distance = (level[low] - level[lca]) + (level[high] - level[lca]);
                if total_distance % 2 != 0 {
                    out_line!(0);
                    continue;
                }
                let moves_low = total_distance / 2;

                // Same logic as above. All the _other_ vertices in the `meeting_point` subtree
                // will have equal distance.
                let meeting_point = lca_table.up(low, moves_low);
                let just_below_from_b = lca_table.up(low, moves_low - 1);
                let other_vertices_in_subtree =
                    subtree_size[meeting_point] - subtree_size[just_below_from_b];
                out_line!(other_vertices_in_subtree);
            }
        }
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
    output().flush();
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
