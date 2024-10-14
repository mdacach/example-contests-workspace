//{"name":"C. Smilo and Monsters","group":"Codeforces - Codeforces Round 907 (Div. 2)","url":"https://codeforces.com/problemset/problem/1891/C","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n4\n1 3 1 1\n4\n1 2 1 1\n6\n3 2 1 5 2 4\n2\n1 6\n","output":"4\n4\n11\n5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CSmiloAndMonsters"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let mut monsters = input.read_u64_vec(n);
    monsters.sort();

    let mut prefix_sum = vec![0; n];
    prefix_sum[0] = monsters[0];
    for i in 1..n {
        prefix_sum[i] = prefix_sum[i - 1] + monsters[i];
    }

    // Second idea: Let's Binary Search the number of hordes killed with single combo attacks.as

    let mut is_ok = |hordes| {
        let mut build_combo = 0;
        let mut must_kill_total = 0;
        for i in 0..n {
            if i < n - hordes {
                build_combo += monsters[i];
            } else {
                must_kill_total += monsters[i];
            }
        }
        build_combo >= must_kill_total
    };

    let mut left = 0; // 0 is always possible, as we can fully use normal attacks for every horde.
    let mut right = n; // n is never possible, as the combo counter starts at 0;
    while right - left > 1 {
        let middle = left + (right - left) / 2;
        if is_ok(middle) {
            left = middle;
        } else {
            right = middle;
        }
    }

    let last_horde = n - left - 1;
    // At this point, we know that we can kill `left` hordes with `left` combo attacks.
    // Now, we may be able to kill the next horde by using both normal attacks and a combo.

    // We must check if there's any leftover kills up-to here.

    let mut before = 0;
    for i in 0..last_horde {
        before += monsters[i];
    }

    let mut after = 0;
    for i in (last_horde + 1)..n {
        after += monsters[i];
    }

    let mut answer = 0;
    answer += left as u64; // The original combo attacks.
    answer += after; // These were needed to build the combos.

    if before >= after {
        // We know that we can do all the combos afterward without using this horde.
        // So we are free to combo it partially.

        let monsters_here = monsters[last_horde];
        let surplus = before - after;
        let total = monsters_here + surplus;
        if total == 1 {
            answer += 1;
        } else if total >= 1 {
            answer += (total + 1) / 2 + 1;
        }
    } else {
        // We need to get some monsters from here to build the combo needed afterward.
        let needed = after - before;
        let remaining_here = monsters[last_horde] - needed;
        if remaining_here == 1 {
            answer += 1;
        } else if remaining_here > 1 {
            answer += (remaining_here + 1) / 2 + 1;
        }
    }

    out.print_line(answer);

    // Previous idea: implementation.
    /*
    let mut combo_attacks = 0;
    let mut other_attacks = 0;
    let mut used_for_building_combo = 0;
    let mut current_target_for_combo = n - 1;
    loop {
        // The plan is to kill some of the smaller hordes with normal attacks, than kill this current
        // horde with a single combo attack.

        let still_left_to_build_combo = {
            // All the possible monsters from non-used hordes.
            let total_sum = prefix_sum[current_target_for_combo - 1];
            // Minus the ones we have already reserved.
            if total_sum >= used_for_building_combo {
                total_sum - used_for_building_combo
            } else {
                0
            }
        };

        // If this is the case, we can kill this whole horde with a single combo attack.
        if usable >= monsters[current_target_for_combo] {
            combo_attacks += 1;

            // After this attack, we know that we already used some of our combo-building potential.
            used_for_building_combo += monsters[current_target_for_combo];
            // And we target the next horde, unless we are out of options.
            if usable == monsters[current_target_for_combo] {
                out.print_line(combo_attacks + used_for_building_combo);
                return;
            }

            current_target_for_combo -= 1;
            if current_target_for_combo == 0 {
                unreachable!("We would need to attack singly at least one time, as combo counter starts with 0");
            }
        } else {
            let this_horde = monsters[current_target_for_combo];
            let combo_potential = this_horde + usable;
            // We have this_horde + usable of potential.
            // We can build a combo of combo_potential / 2, and then perform one combo to kill the others.
            if combo_potential > 1 {
                combo_attacks += 1;
            }
            other_attacks += (combo_potential + 1) / 2;

            // And then we are done.
            out.print_line(combo_attacks + other_attacks + used_for_building_combo);
            return;
        }
    }
     */
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
