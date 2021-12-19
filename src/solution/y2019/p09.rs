use aocio::aocio;
use itertools::Itertools;

use crate::solution::aoc_test;

use super::intcode::Program;

#[aocio]
pub fn small(a: Vec<i64, ",">) -> String {
    let mut prog = Program::new(a);
    prog.write(1);
    prog.read_all().iter().map(|x| x.to_string()).join(",")
}

#[aocio]
pub fn large(a: Vec<i64, ",">) -> String {
    let mut prog = Program::new(a);
    prog.write(2);
    prog.read_all().iter().map(|x| x.to_string()).join(",")
}

aoc_test!(
    2019,
    9,
    "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99",
    "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99"
);
