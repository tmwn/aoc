use aocio::aocio;
use itertools::Itertools;

use crate::solution::{aoc_test, y2019::intcode::Program};



#[aocio]
pub fn small(a: Vec<i64, ",">) -> i64 {
    let mut res = 0;
    for perm in [0, 1, 2, 3, 4].iter().permutations(5) {
        let mut v = 0;
        for i in 0..5 {
            let mut prog = Program::new(a.clone());
            prog.write(*perm[i]);
            prog.write(v);
            prog.run_until_halt();
            v = prog.read().unwrap();
        }
        res = res.max(v);
    }
    res
}

#[aocio]
pub fn large(a: Vec<i64, ",">) -> i64 {
    let mut res = 0;
    for perm in (5..10).permutations(5) {
        let mut progs = vec![];
        for i in 0..5 {
            let mut prog = Program::new(a.clone());
            prog.write(perm[i]);
            progs.push(prog);
        }
        progs[0].write(0);

        let mut val = 0;
        loop {
            let mut halt_count = 0;
            for i in 0..5 {
                if !progs[i].running() {
                    halt_count += 1;
                }
                while let Some(x) = progs[i].read() {
                    progs[(i + 1) % 5].write(x);
                    if i == 4 {
                        val = x;
                    }
                }
            }
            if halt_count == 5 {
                break;
            }
        }
        res = res.max(val);
    }
    res
}

aoc_test!(
    2019,
    7,
    "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0",
    43210
);

aoc_test!(
    2019,
    7,
    "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,
    27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5",,
    139629729,
    l1
);
