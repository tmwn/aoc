use aocio::aocio;

use crate::solution::aoc_test;

#[aocio]
pub fn small(mut a: Vec<Vec<char, "">>) -> i32 {
    let h = a.len();
    let w = a[0].len();
    let mut do_move = vec![vec![false; w]; h];
    for i in 0.. {
        let mut moved = false;
        for i in 0..h {
            for j in 0..w {
                do_move[i][j] = false;
            }
        }
        for i in 0..h {
            for j in 0..w {
                if a[i][j] != '>' {
                    continue;
                }
                let nj = (j + 1) % w;
                if a[i][nj] == '.' {
                    do_move[i][j] = true;
                }
            }
        }
        for i in 0..h {
            for j in 0..w {
                if do_move[i][j] {
                    moved = true;
                    let nj = (j + 1) % w;
                    a[i][j] = '.';
                    a[i][nj] = '>';
                }
            }
        }

        for i in 0..h {
            for j in 0..w {
                do_move[i][j] = false;
            }
        }
        for i in 0..h {
            for j in 0..w {
                if a[i][j] != 'v' {
                    continue;
                }
                let ni = (i + 1) % h;
                if a[ni][j] == '.' {
                    do_move[i][j] = true;
                }
            }
        }
        for i in 0..h {
            for j in 0..w {
                if do_move[i][j] {
                    moved = true;
                    let ni = (i + 1) % h;
                    a[i][j] = '.';
                    a[ni][j] = 'v';
                }
            }
        }
        if !moved {
            return i + 1;
        }
    }
    panic!()
}

#[aocio]
pub fn large(_: Vec<String>) -> i32 {
    panic!("no puzzle")
}

aoc_test!(
    2021,
    25,
    "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>",
    58
);
