use itertools::Itertools;
use std::str::FromStr;

pub fn small(a: Vec<Line>) -> i32 {
    let mut res = 0;
    for line in a {
        for o in line.output {
            match o.len() {
                2 | 3 | 4 | 7 => res += 1,
                _ => (),
            }
        }
    }
    res
}

pub fn large(a: Vec<Line>) -> i32 {
    let mut res = 0;
    for line in a {
        res += solve(line);
    }
    res
}

const DIGITS: [&str; 10] = [
    "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
];

fn get(letters: &str, permutation: &[u8]) -> Option<i32> {
    let mut mask = vec![];
    for c in letters.bytes() {
        mask.push((permutation[(c - b'a') as usize] + b'a') as char);
    }
    mask.sort_unstable();
    let s: String = mask.iter().collect();
    for (i, x) in DIGITS.iter().enumerate() {
        if *x == s {
            return Some(i as i32);
        }
    }
    None
}

fn solve(line: Line) -> i32 {
    'outer: for is in (0..7).permutations(7) {
        for x in line.input.iter() {
            if get(x, &is).is_none() {
                continue 'outer;
            }
        }
        let mut res = 0;
        for x in line.output.iter() {
            res = res * 10 + get(x, &is).unwrap();
        }
        return res;
    }
    panic!("no solution")
}

pub struct Line {
    input: Vec<String>,
    output: Vec<String>,
}

impl FromStr for Line {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ss: Vec<_> = s.split(" | ").collect();
        let input = ss[0].split(' ').map(ToOwned::to_owned).collect();
        let output = ss[1].split(' ').map(ToOwned::to_owned).collect();
        Ok(Line { input, output })
    }
}

#[cfg(test)]
mod tests {
    use crate::solution::solve;

    const INPUT: &str = r#"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"#;
    #[test]
    fn small() {
        assert_eq!(solve(INPUT, 8, false).unwrap(), "26");
    }
    #[test]
    fn large() {
        assert_eq!(solve(INPUT, 8, true).unwrap(), "61229");
    }
}
