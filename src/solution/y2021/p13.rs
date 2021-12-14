use std::str::FromStr;

use crate::solution::{aoc_test, util::point::Point};

pub fn small((mut ps, instrs): (Vec<Point<i32>>, Vec<Instr>)) -> usize {
    fold(&mut ps, &instrs[0]);
    ps.sort();
    ps.dedup();
    ps.len()
}

pub fn large((mut ps, instrs): (Vec<Point<i32>>, Vec<Instr>)) -> String {
    for i in instrs {
        fold(&mut ps, &i);
    }
    let x_max = ps.iter().fold(0, |m, p| m.max(p.x));
    let y_max = ps.iter().fold(0, |m, p| m.max(p.y));
    ps.sort();
    let mut res = String::new();
    for y in 0..=y_max {
        for x in 0..=x_max {
            if ps.binary_search(&Point { x, y }).is_ok() {
                res.push('#');
            } else {
                res.push('.');
            }
        }
        res.push('\n');
    }
    res.pop();
    res
}

fn fold(ps: &mut Vec<Point<i32>>, instr: &Instr) {
    for p in ps {
        if instr.x && p.x > instr.val {
            p.x = instr.val * 2 - p.x;
        }
        if !instr.x && p.y > instr.val {
            p.y = instr.val * 2 - p.y;
        }
    }
}

pub struct Instr {
    x: bool,
    val: i32,
}

impl FromStr for Instr {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ss: Vec<_> = s.split('=').collect();
        let x = ss[0].as_bytes()[ss[0].len() - 1] == b'x';
        let val = ss[1].parse()?;
        Ok(Instr { x, val })
    }
}

aoc_test!(
    2021,
    13,
    r#"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5"#,
    17,
    r#"#####
#...#
#...#
#...#
#####"#
);
