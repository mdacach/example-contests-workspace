//{"name":"D. Vika and Bonuses","group":"Codeforces - Codeforces Round 885 (Div. 2)","url":"https://codeforces.com/contest/1848/problem/D","interactive":false,"timeLimit":3000,"tests":[{"input":"6\n1 3\n11 3\n0 179\n5 1000000000\n723252212 856168102\n728598293 145725253\n","output":"4\n33\n0\n9999999990\n1252047198518668448\n106175170582793129\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DVikaAndBonuses"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

// First, we need to make some observations:
// 1. If you will take X bonuses, you always want to take all of them _first_.
// This is intuitive because the bonuses do not depend on anything else. Taking the bonus at i+1 or
// i doesn't change its value, it's the same bonus.
//
// 2. At some point, it stops making sense to take bonuses.
// When you take a bonus, you increase your potential discount, but you have less buys to exercise
// it.
//
// This makes us think of Ternary Search, because taking bonuses is better at the beginning, then
// it becomes worse and worse, so this function seems unimodal.
// BUT, we can't guarantee that this function is _strictly_ unimodal. If there's two adjacent
// positions with the same value, it will break the search.
//
// So we are kind of stuck... But we know we must answer the queries very fast (because there's
// 10^5 tests, with no upperbound on S or K)... What else can we fix?
// We only have a finite amount of last digits to consider. If we can find the maximum discount if
// the number ends in 0, find the maximum discount if the number ends in 1...
// We can just compare them all.
// So let's try that.
// 0. If the number ends in 0, we won't ever increase the bonus, so might as well just never do that.
// 1. If the number ends in 1, we may add bonuses and the last digit becomes 2 -> 4 -> 8 -> 6 -> 2 ->
// ... So we actually can't really get back to 1... So we should just buy everything with this
// current bonus too.
// 2. If the number ends in 2, we may add bonuses indefinitely (respecting the cycle)
// 2 -> 4 -> 8 -> 6 -> 2 -> 4 -> 8 -> 6 -> 2 -> ...
// ^                   ^                   ^
// We may stop in any of those places. And we can't really test them all (because it's O(K))...
// Let's go faster. The same idea applies. We are starting with some bonus and some buys left.
// If you add bonus, this is good because your discount will be great when you end up buying stuff.
// But of course you don't wanna add _too much_. This is the same reasoning that made us consider
// the Ternary Search above, so let's try that again.
// Does Ternary Search work for _this_ problem (where the last digit is fixed?).
// The function is unimodal, sure (it first increases, possibly plateaus, and then decreases);
// Is it _strictly_ unimodal, though? Are there adjacent equal values?
// Let's consider the amound of "cycles" we will take (starting at a 2 and getting back to 2 is a
// cycle, so 4 -> 8 -> 6 -> 2). What happens if we take `x` cycles?
// For each cycle, we "waste" 4 buys (because the size of the cycle is 4).
// For each cycle, we increase our bonus by 20 (4 + 8 + 6 + 2).
//
// `total_discount` = (`prior_bonus` + 20 * `x`) * (`all_buys` - 4 * `x`)
//                             ^^^^^^^                    ^^^^^^^
//                       bonus after all cycles        remaining buys
//  If you multiply that, you see that this is a quadratic formula (thus a parabola).
//  In a parabola, you never have two adjacent equal values -> a parabola is strictly unimodal.
//  So in _this_ function, we can safely apply Ternary Search.
//
//  So do that for each reachable last digit (2, 4, 6, 8 - because they are part of the cycle) and
//  compare all the possible maximums.
//
//  TLDR: If you restrain your initial function, you make it ternary-searchable. This is very
//  interesting!
//
//
//
//
//
// My thoughts:
// Had the couple of observations rather quickly. Actually thought about fixing the last digit, bet
// _before_ the ternary search. So when I thought about ternary search, I tried it on the original
// function that didn't work. It's _possible_ I would have passed this in contest (if I wasn't
// stuck at C, that is) but it would be a slim chance. Very interesting problem anyway, made me
// refresh on ternary search (and binary searching it). I wish there were more resources on that
// topic.
// Also should refresh on exchange argument (still didn't do that!).
fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let (s, k) = (input.read_long(), input.read_long());

    let last_digit = s % 10;
    if last_digit == 0 {
        // You can never add a bonus.
        out.print_line(k * s);
        return;
    } else if last_digit == 5 {
        // You may add a bonus once.
        let adding_once = (s + 5) * (k - 1);
        let adding_zero = s * k;
        out.print_line(std::cmp::max(adding_once, adding_zero));
        return;
    }

    let mut best_answer = i64::MIN;
    let mut current_sum = s;
    for first_moves in 0..std::cmp::min(10, k) {
        let last_digit = current_sum % 10;

        let mut bonuses_left = k - first_moves;
        best_answer.maxim(current_sum * bonuses_left);
        if vec![2, 4, 8, 6].contains(&last_digit) {
            let after_x_cycles = |x: i64| {
                let cycle_sum = 20;
                (current_sum + x * cycle_sum) * (bonuses_left - 4 * x)
            };

            let mut left = 0;
            let mut right = bonuses_left / 4;
            while right - left > 1 {
                let middle = left + (right - left) / 2;
                let this = after_x_cycles(middle);
                let next = after_x_cycles(middle + 1);
                if next > this {
                    left = middle;
                } else {
                    right = middle;
                }
            }

            let best_here = after_x_cycles(left + 1);
            best_answer.maxim(best_here);
        }

        current_sum += last_digit;
    }
    out.print_line(best_answer);
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
