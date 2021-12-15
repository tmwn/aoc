use aocio::aocio;
use itertools::Itertools;

#[aocio]
pub fn small(a: Vec<Tuple<Vec<String, " ">, "|", Vec<String, " ">>>) -> i32 {
    let mut res = 0;
    for (_, output) in a {
        for o in output {
            match o.len() {
                2 | 3 | 4 | 7 => res += 1,
                _ => (),
            }
        }
    }
    res
}

#[aocio]
pub fn large(a: Vec<Tuple<Vec<String, " ">, "|", Vec<String, " ">>>) -> i32 {
    let mut res = 0;
    for (input, output) in a {
        res += solve(input, output);
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

fn solve(input: Vec<String>, output: Vec<String>) -> i32 {
    'outer: for is in (0..7).permutations(7) {
        for x in input.iter() {
            if get(x, &is).is_none() {
                continue 'outer;
            }
        }
        let mut res = 0;
        for x in output.iter() {
            res = res * 10 + get(x, &is).unwrap();
        }
        return res;
    }
    panic!("no solution")
}

#[cfg(test)]
mod tests {
    use super::super::solve;

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
