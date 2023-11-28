//{"name":"C2. Powering the Hero (hard version)","group":"Codeforces - Codeforces Round 855 (Div. 3)","url":"https://codeforces.com/contest/1800/problem/C2","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n5\n3 3 3 0 0\n6\n0 3 3 0 0 3\n7\n1 2 3 0 4 5 0\n7\n1 2 5 0 4 3 0\n5\n3 1 0 0 4\n","output":"6\n6\n8\n9\n4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"C2PoweringTheHeroHardVersion"}}}

use std::collections::BinaryHeap;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

// Suppose you are playing the game. If you get a hero card, you want to have a big bonus ready to
// pair with it. If you get another hero card, you want another big bonus. And so on...
// When you get heroes, you want to have big bonuses ready.
// But it's not that simple - you can't change the order of your bonus deck.
// The observation is: it doesn't matter if your biggest bonus pairs with your first hero, or if it
// pairs with your second hero. They all sum to the same power afterall.
// So when you are getting two hero cards, you want to have your 2 biggest bonuses _in any order_
// in the deck.
//
// No matter which order you get the bonuses, if you receive a hero, you will be able to give it
// the biggest bonus you have seen. When you receive another one, you will be able to give them
// both the two biggest bonuses you have seen - you don't know the _order_, but it actually
// doesn't matter.
//
// Thus the solution is to simply maintain the biggest bonuses you have seen, and when you get a
// hero, use the biggest on him. Note that this is not a valid simulation of how the game will
// actually be played, but it _will_ amount to the same total army power, so it is a solution
// anyway.
fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let cards = input.read_long_vec(n);

    let mut biggest = BinaryHeap::new();
    let mut answer = 0;
    for card in cards {
        if card == 0 {
            if let Some(bonus) = biggest.pop() {
                answer += bonus;
            }
        } else {
            biggest.push(card);
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
