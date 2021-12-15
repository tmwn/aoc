use aocio::aocio;

use crate::solution::aoc_test;

#[aocio]
pub fn small(a: Vec<Tuple<String, " ", i32>>) -> i32 {
    let (mut x, mut y) = (0i32, 0i32);
    for (s, k) in a {
        match s.as_ref() {
            "forward" => y += k,
            "down" => x += k,
            "up" => x -= k,
            _ => panic!(),
        }
    }
    x * y
}

#[aocio]
pub fn large(a: Vec<Tuple<String, " ", i32>>) -> i32 {
    let mut aim = 0;
    let mut depth = 0;
    let mut pos = 0;
    for (s, k) in a {
        match s.as_ref() {
            "down" => aim += k,
            "up" => aim -= k,
            "forward" => {
                pos += k;
                depth += k * aim
            }
            _ => panic!(),
        }
    }
    pos * depth
}

aoc_test!(
    2021,
    2,
    "forward 5
down 5
forward 8
up 3
down 8
forward 2",
    150,
    900
);
