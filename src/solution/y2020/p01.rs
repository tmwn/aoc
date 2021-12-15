use aocio::aocio;

use crate::solution::aoc_test;

#[aocio]
pub fn small(a: Vec<usize>) -> usize {
    let mut b = vec![false; 2021];
    for x in a {
        b[x] = true;
    }
    for i in 0..2020 {
        if b[i] && b[2020 - i] {
            return i * (2020 - i);
        }
    }
    0
}

#[aocio]
pub fn large(a: Vec<usize>) -> usize {
    let mut b = vec![false; 2021];
    for x in a {
        b[x] = true;
    }
    for i in 0..2020 {
        for j in 0..(2020 - i) {
            if b[i] && b[j] && b[2020 - i - j] {
                return i * j * (2020 - i - j);
            }
        }
    }
    0
}

aoc_test!(
    2021,
    1,
    "199
200
208
210
200
207
240
269
260
263",
    7,
    5
);
