pub fn small(a: Vec<String>) -> i32 {
    let mut res = 0;
    for line in a {
        res += match analyze(&line) {
            State::Incomplete(_) => 0,
            State::Illegal(')') => 3,
            State::Illegal(']') => 57,
            State::Illegal('}') => 1197,
            State::Illegal('>') => 25137,
            x => panic!("{:?}", x),
        };
    }
    res
}

const PARENS: &'static str = "(){}<>[]";

#[derive(Debug)]
enum State {
    Illegal(char),
    Incomplete(Vec<char>),
}

fn analyze(line: &str) -> State {
    let mut to_closing = std::collections::HashMap::<char, char>::new();
    for i in 0..4 {
        to_closing.insert(
            PARENS.as_bytes()[i * 2] as char,
            PARENS.as_bytes()[i * 2 + 1] as char,
        );
    }
    let mut expect = vec![];
    for c in line.chars() {
        if let Some(closing) = to_closing.get(&c) {
            expect.push(*closing);
        } else {
            if expect.pop() != Some(c) {
                return State::Illegal(c);
            }
        }
    }
    State::Incomplete(expect)
}

pub fn large(a: Vec<String>) -> i64 {
    let mut scores = vec![];
    for line in a {
        if let State::Incomplete(stack) = analyze(&line) {
            let mut score = 0;
            for c in stack.iter().rev() {
                score = score * 5
                    + match c {
                        ')' => 1,
                        ']' => 2,
                        '}' => 3,
                        '>' => 4,
                        x => panic!("{}", x),
                    };
            }
            scores.push(score);
        }
    }
    scores.sort();
    scores[scores.len() / 2]
}

#[cfg(test)]
mod tests {
    use crate::solution::solve;

    const INPUT: &'static str = r#"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"#;

    #[test]
    fn small() {
        assert_eq!(solve(INPUT, 10, false).unwrap(), "26397");
    }
    #[test]
    fn large() {
        assert_eq!(solve(INPUT, 10, true).unwrap(), "288957");
    }
}
