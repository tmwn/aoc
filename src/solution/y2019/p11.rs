use aocio::aocio;

use crate::solution::{aoc_test, util::grid};

use super::intcode::Program;

#[aocio]
pub fn small(input: Vec<i64, ",">) -> i32 {
    let mut prog = Program::new(input);
    let n = 100;
    let mut visited = vec![vec![false; n]; n];
    let mut white = vec![vec![false; n]; n];
    let mut x = n / 2;
    let mut y = n / 2;
    let mut d = 0;
    prog.write(0);
    while prog.running() {
        white[x][y] = prog.read().unwrap() == 1;
        visited[x][y] = true;
        if prog.read().unwrap() == 0 {
            d = (d + 1) % 4;
        } else {
            d = (d + 3) % 4;
        }
        x = x.wrapping_add(grid::DIR4[d].0 as usize);
        y = y.wrapping_add(grid::DIR4[d].1 as usize);
        prog.write(if white[x][y] { 1 } else { 0 });
    }
    let mut res = 0;
    for row in visited {
        for x in row {
            if x {
                res += 1;
            }
        }
    }
    res
}

#[aocio]
pub fn large(a: Vec<i64, ",">) -> String {
    let mut prog = Program::new(a);

    let h = 7;
    let w = 43;
    let mut white = vec![vec![false; w]; h];
    let mut x = 0;
    let mut y = 0;
    white[x][y] = true;
    let mut d = 2;
    prog.write(1);
    while prog.running() {
        white[x][y] = prog.read().unwrap() == 1;
        if prog.read().unwrap() == 0 {
            d = (d + 1) % 4;
        } else {
            d = (d + 3) % 4;
        }
        x = x.wrapping_add(grid::DIR4[d].0 as usize);
        y = y.wrapping_add(grid::DIR4[d].1 as usize);
        prog.write(if white[x][y] { 1 } else { 0 });
    }
    let mut res = String::new();
    for x in 0..h {
        for y in 0..w {
            res.push(if white[x][y] { '#' } else { '.' });
        }
        res.push('\n');
    }
    res
}

aoc_test!(
    2019, 11, "3,8,1005,8,301,1106,0,11,0,0,0,104,1,104,0,3,8,102,-1,8,10,1001,10,1,10,4,10,108,1,8,10,4,10,102,1,8,28,1006,0,98,3,8,102,-1,8,10,101,1,10,10,4,10,1008,8,1,10,4,10,101,0,8,54,2,1001,6,10,1,108,1,10,3,8,1002,8,-1,10,101,1,10,10,4,10,1008,8,0,10,4,10,1002,8,1,84,3,8,102,-1,8,10,1001,10,1,10,4,10,108,1,8,10,4,10,101,0,8,105,1006,0,94,2,7,20,10,2,5,7,10,3,8,1002,8,-1,10,1001,10,1,10,4,10,1008,8,0,10,4,10,102,1,8,139,1006,0,58,2,1003,16,10,1,6,10,10,3,8,1002,8,-1,10,101,1,10,10,4,10,1008,8,0,10,4,10,102,1,8,172,2,107,12,10,3,8,102,-1,8,10,1001,10,1,10,4,10,108,0,8,10,4,10,101,0,8,197,1006,0,34,3,8,102,-1,8,10,101,1,10,10,4,10,1008,8,1,10,4,10,102,1,8,223,1006,0,62,3,8,102,-1,8,10,101,1,10,10,4,10,1008,8,1,10,4,10,1001,8,0,248,1,7,7,10,1006,0,64,2,1008,5,10,3,8,1002,8,-1,10,1001,10,1,10,4,10,108,0,8,10,4,10,102,1,8,280,101,1,9,9,1007,9,997,10,1005,10,15,99,109,623,104,0,104,1,21102,1,387508351636,1,21101,318,0,0,1106,0,422,21102,1,838480007948,1,21101,0,329,0,1106,0,422,3,10,104,0,104,1,3,10,104,0,104,0,3,10,104,0,104,1,3,10,104,0,104,1,3,10,104,0,104,0,3,10,104,0,104,1,21101,0,235190525123,1,21101,0,376,0,1105,1,422,21101,0,106505084123,1,21101,0,387,0,1106,0,422,3,10,104,0,104,0,3,10,104,0,104,0,21101,0,838324605292,1,21102,1,410,0,1105,1,422,21102,709496668940,1,1,21102,421,1,0,1105,1,422,99,109,2,22101,0,-1,1,21102,1,40,2,21101,0,453,3,21102,443,1,0,1106,0,486,109,-2,2105,1,0,0,1,0,0,1,109,2,3,10,204,-1,1001,448,449,464,4,0,1001,448,1,448,108,4,448,10,1006,10,480,1102,1,0,448,109,-2,2106,0,0,0,109,4,2101,0,-1,485,1207,-3,0,10,1006,10,503,21102,0,1,-3,22102,1,-3,1,21201,-2,0,2,21101,1,0,3,21102,1,522,0,1106,0,527,109,-4,2105,1,0,109,5,1207,-3,1,10,1006,10,550,2207,-4,-2,10,1006,10,550,21202,-4,1,-4,1106,0,618,22102,1,-4,1,21201,-3,-1,2,21202,-2,2,3,21102,569,1,0,1106,0,527,21202,1,1,-4,21101,0,1,-1,2207,-4,-2,10,1006,10,588,21101,0,0,-1,22202,-2,-1,-2,2107,0,-3,10,1006,10,610,22101,0,-1,1,21101,0,610,0,106,0,485,21202,-2,-1,-2,22201,-4,-2,-4,109,-5,2106,0,0", 1883, "..##..###..#..#..##..#..#.###..####.#..#...
.#..#.#..#.#..#.#..#.#..#.#..#.#....#..#...
.#..#.#..#.#..#.#....#..#.#..#.###..####...
.####.###..#..#.#.##.#..#.###..#....#..#...
.#..#.#....#..#.#..#.#..#.#.#..#....#..#...
.#..#.#.....##...###..##..#..#.#....#..#...
...........................................
"
);
