use std::collections::{BTreeMap, HashMap};

use aocio::aocio;

pub fn small(s: String) -> i64 {
    solve(s.parse().unwrap()).0
}

pub fn large(s: String) -> i64 {
    solve(s.parse().unwrap()).1
}

#[aocio]
pub fn solve(prog: Vec<Vec<Tuple<String, " ", String, " ", String>>, "inp w">) -> (i64, i64) {
    let max = 10_000_000;
    let mut dp = BTreeMap::<i32, (i64, i64)>::new();
    dp.insert(0, (0, 0));

    let mut n_dp = BTreeMap::<i32, (i64, i64)>::new();
    for p in prog {
        n_dp.clear();
        for (z, val) in dp.iter() {
            for w in 1i32..=9 {
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

fn run(reg: &mut [i32; 4], prog: &Vec<(String, String, String)>) {
    for (instr, left, right) in prog {
        let left = (left.as_bytes()[0] - b'w') as usize;
        let right = match right.as_bytes()[0] {
            x @ b'w'..=b'z' => reg[(x - b'w') as usize],
            _ => right.parse().unwrap(),
        };
        match instr.as_str() {
            "add" => reg[left] = reg[left] + right,
            "mul" => reg[left] = reg[left] * right,
            "div" => reg[left] = reg[left] / right,
            "mod" => reg[left] = reg[left] % right,
            "eql" => reg[left] = if reg[left] == right { 1 } else { 0 },
            _ => panic!(),
        }
    }
}
