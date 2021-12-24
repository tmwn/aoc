use std::{collections::BTreeMap, str::FromStr};

use aocio::aocio;

use crate::solution::aoc_test;

pub fn small(s: String) -> i64 {
    solve(s.parse().unwrap()).0
}

pub fn large(s: String) -> i64 {
    solve(s.parse().unwrap()).1
}

#[aocio]
pub fn solve(prog: Vec<Vec<Tuple<Instr, " ", Pos, " ", Pos>>, "inp w">) -> (i64, i64) {
    let max = 7_000_000;
    let mut dp = BTreeMap::<i32, (i64, i64)>::new();
    let mut n_dp = dp.clone();

    dp.insert(0, (0, 0));

    for p in prog {
        n_dp.clear();
        for (z, val) in dp.iter() {
            for w in 1..=9 {
                let n_val = (val.0 * 10 + w as i64, val.1 * 10 + w as i64);

                let mut reg = [w, 0, 0, *z];
                run(&mut reg, &p);
                let n_z = reg[3];
                if n_z < max && n_z >= 0 {
                    let nxt_val = n_dp.get_mut(&n_z);
                    if let Some(nxt_val) = nxt_val {
                        *nxt_val = (nxt_val.0.max(n_val.0), nxt_val.1.min(n_val.1));
                    } else {
                        n_dp.insert(n_z, n_val);
                    }
                }
            }
        }
        std::mem::swap(&mut dp, &mut n_dp);
    }
    *dp.get(&0).unwrap()
}

enum Instr {
    Add,
    Mul,
    Div,
    Mod,
    Eql,
}

impl FromStr for Instr {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "add" => Instr::Add,
            "mul" => Instr::Mul,
            "div" => Instr::Div,
            "mod" => Instr::Mod,
            "eql" => Instr::Eql,
            _ => panic!(),
        })
    }
}

enum Pos {
    Index(usize),
    Immediate(i32),
}

impl FromStr for Pos {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.as_bytes()[0] {
            x @ b'w'..=b'z' => Pos::Index((x - b'w') as usize),
            _ => Pos::Immediate(s.parse().unwrap()),
        })
    }
}

fn run(reg: &mut [i32], prog: &Vec<(Instr, Pos, Pos)>) {
    for (instr, left, right) in prog {
        let left = match left {
            Pos::Index(i) => *i,
            _ => panic!(),
        };
        let right = match right {
            Pos::Index(i) => reg[*i],
            Pos::Immediate(i) => *i,
        };
        match instr {
            Instr::Add => reg[left] += right,
            Instr::Mul => reg[left] *= right,
            Instr::Div => reg[left] /= right,
            Instr::Mod => reg[left] %= right,
            Instr::Eql => reg[left] = if reg[left] == right { 1 } else { 0 },
        }
    }
}

aoc_test!(
    2021,
    24,
    "inp w
mul x 0
add x z
mod x 26
div z 1
add x 12
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 4
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 11
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 10
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 14
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 12
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -6
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 14
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 15
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 6
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 12
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 16
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -9
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 1
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 14
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 7
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 14
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 8
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -5
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 11
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -9
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 8
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -5
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 3
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -2
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 1
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -7
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 8
mul y x
add z y",
    91398299697996i64,
    41171183141291i64
);
