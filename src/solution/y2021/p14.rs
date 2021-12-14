use std::str::FromStr;

use crate::solution::util::matrix::{self, Matrix};

pub fn small(p: (String, Vec<Rule>)) -> i64 {
    solve(p, 10)
}

pub fn large(p: (String, Vec<Rule>)) -> i64 {
    solve(p, 40)
}

pub fn solve((ps, rules): (String, Vec<Rule>), step: usize) -> i64 {
    let mut cs: Vec<u8> = ps.clone().into();
    for r in &rules {
        cs.push(r.0);
        cs.push(r.1);
        cs.push(r.2);
    }
    cs.sort_unstable();
    cs.dedup();

    let n = cs.len();

    let mut mat: Matrix<i64> = matrix::new(n * n, n * n);
    let mut vec: Vec<i64> = vec![0; n * n];
    for i in 1..ps.len() {
        let a = cs.binary_search(&ps.as_bytes()[i - 1]).unwrap();
        let b = cs.binary_search(&ps.as_bytes()[i]).unwrap();
        vec[a * n + b] += 1;
    }

    for r in &rules {
        let a = cs.binary_search(&r.0).unwrap();
        let b = cs.binary_search(&r.1).unwrap();
        let c = cs.binary_search(&r.2).unwrap();
        mat[a * n + c][a * n + b] = 1;
        mat[c * n + b][a * n + b] = 1;
    }

    let v = matrix::mul_vec(&matrix::pow(&mat, step), &vec);
    let mut cnt = vec![0; n];
    for i in 0..v.len() {
        cnt[i / n] += v[i];
    }
    cnt[cs.binary_search(&ps.as_bytes()[ps.len() - 1]).unwrap()] += 1;
    let max_cnt = cnt.iter().fold(0, |m, x| m.max(*x));
    let min_cnt = cnt
        .iter()
        .fold(i64::MAX, |m, x| if *x == 0 { m } else { m.min(*x) });
    max_cnt - min_cnt
}

#[derive(Debug)]
pub struct Rule(u8, u8, u8);

impl FromStr for Rule {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let a = s.as_bytes()[0];
        let b = s.as_bytes()[1];
        let c = s.as_bytes()[6];
        Ok(Rule(a, b, c))
    }
}

crate::solution::aoc_test!(
    2021,
    14,
    r"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
",
    1588,
    2188189693529i64
);
