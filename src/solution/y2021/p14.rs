use std::str::FromStr;

use crate::solution::{parse, Parse};

pub fn small(p: Problem) -> i64 {
    solve(p, 10)
}
pub fn large(p: Problem) -> i64 {
    solve(p, 40)
}

pub fn solve(Problem { s, rules }: Problem, step: usize) -> i64 {
    let mut cs: Vec<u8> = s.as_bytes().clone().into();
    for r in &rules {
        cs.push(r.0);
        cs.push(r.1);
        cs.push(r.2);
    }
    cs.sort();
    cs.dedup();

    let n = cs.len();

    let mut mat: Vec<Vec<i64>> = vec![vec![0; n * n]; n * n];
    let mut vec: Vec<i64> = vec![0; n * n];
    for i in 1..s.len() {
        let a = cs.binary_search(&s.as_bytes()[i - 1]).unwrap();
        let b = cs.binary_search(&s.as_bytes()[i]).unwrap();
        vec[a * n + b] += 1;
    }

    for r in &rules {
        let a = cs.binary_search(&r.0).unwrap();
        let b = cs.binary_search(&r.1).unwrap();
        let c = cs.binary_search(&r.2).unwrap();
        mat[a * n + c][a * n + b] = 1;
        mat[c * n + b][a * n + b] = 1;
    }

    let v = mul(pow(mat, step), vec);
    let mut cnt = vec![0; n];
    for i in 0..v.len() {
        cnt[i / n] += v[i];
    }
    cnt[cs.binary_search(&s.as_bytes()[s.len() - 1]).unwrap()] += 1;
    let max_cnt = cnt.iter().fold(0, |m, x| m.max(*x));
    let min_cnt = cnt
        .iter()
        .fold(i64::MAX, |m, x| if *x == 0 { m } else { m.min(*x) });
    max_cnt - min_cnt
}

fn mul(m: Vec<Vec<i64>>, v: Vec<i64>) -> Vec<i64> {
    let n = v.len();
    let mut res = vec![0; n];
    for i in 0..n {
        for j in 0..n {
            res[i] += m[i][j] * v[j];
        }
    }
    res
}

fn mul_mat(m1: Vec<Vec<i64>>, m2: Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let n = m1.len();
    let mut res = vec![vec![0; n]; n];
    for i in 0..n {
        for k in 0..n {
            if m1[i][k] == 0 {
                continue;
            }
            for j in 0..n {
                res[i][j] += m1[i][k] * m2[k][j];
            }
        }
    }

    res
}

fn pow(m: Vec<Vec<i64>>, k: usize) -> Vec<Vec<i64>> {
    dbg!(k);
    if k == 1 {
        return m;
    }
    if k % 2 == 1 {
        return mul_mat(pow(m.clone(), k - 1), m);
    } else {
        let mm = pow(m, k / 2);
        mul_mat(mm.clone(), mm)
    }
}

pub struct Problem {
    s: String,
    rules: Vec<Rule>,
}

impl Parse for Problem {
    fn parse(s: &str) -> Self {
        let mut ss = s.split("\n\n");
        let s = ss.next().unwrap().trim();
        let rules = parse(ss.next().unwrap().trim());
        Self {
            s: s.to_string(),
            rules,
        }
    }
}

#[derive(Debug)]
struct Rule(u8, u8, u8);

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
