use aocio::aocio;

use crate::solution::aoc_test;

use super::intcode::Program;

#[aocio]
pub fn small(mut a: Vec<i32, ",">) -> i32 {
    if std::env::var("PROD").is_ok() {
        a[1] = 12;
        a[2] = 2;
    }
    let mut prog = Program::new(a);
    prog.run()
}

#[aocio]
pub fn large(mut a: Vec<i32, ",">) -> i32 {
    for noun in 0..100 {
        for verb in 0..100 {
            a[1] = noun;
            a[2] = verb;
            let mut prog = Program::new(a.clone());
            let v = prog.run();
            if v == 19690720 {
                return noun * 100 + verb;
            }
        }
    }
    panic!();
}

aoc_test!(2019, 2, "1,9,10,3,2,3,11,0,99,30,40,50", 3500);
