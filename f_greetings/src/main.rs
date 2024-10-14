//{"name":"F. Greetings","group":"Codeforces - Codeforces Round 918 (Div. 4)","url":"https://codeforces.com/problemset/problem/1915/F","interactive":false,"timeLimit":5000,"tests":[{"input":"5\n2\n2 3\n1 4\n6\n2 6\n3 9\n4 5\n1 8\n7 10\n-2 100\n4\n-10 10\n-5 5\n-12 12\n-13 13\n5\n-4 9\n-2 5\n3 4\n6 7\n8 10\n4\n1 2\n3 4\n5 6\n7 8\n","output":"1\n9\n6\n4\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FGreetings"}}}

use algo_lib::collections::treap_map::TreapSet;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use std::cmp::Ordering;

type PreCalc = ();

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone)]
enum Type {
    Start,
    End,
}

#[derive(Ord, Eq, PartialEq, Debug, Copy, Clone)]
struct Event {
    start: i64,
    end: i64,
    _type: Type,
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (&self._type, &other._type) {
            (Type::Start, Type::Start) => self.start.partial_cmp(&other.start),
            (Type::Start, Type::End) => self.start.partial_cmp(&other.end),
            (Type::End, Type::Start) => self.end.partial_cmp(&other.start),
            (Type::End, Type::End) => self.end.partial_cmp(&other.end),
        }
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();

    let mut events = vec![];
    for _ in 0..n {
        let start = input.read_long();
        let end = input.read_long();
        events.push(Event {
            start,
            end,
            _type: Type::Start,
        });
        events.push(Event {
            start,
            end,
            _type: Type::End,
        });
    }

    events.sort();

    let mut currently_open = TreapSet::new();

    let mut answer = 0;
    for Event { start, end, _type } in events {
        match _type {
            Type::Start => {
                currently_open.insert(start);
            }
            Type::End => {
                // Will cover this event.
                let original_start = start;
                answer += currently_open.less(&original_start);

                currently_open.remove(&original_start);
            }
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
