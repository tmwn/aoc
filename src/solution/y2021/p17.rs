use aocio::aocio;

use crate::solution::aoc_test;

pub fn small(s: String) -> i32 {
    solve(s.parse().unwrap()).0
}

pub fn large(s: String) -> i32 {
    solve(s.parse().unwrap()).1
}

#[aocio]
fn solve(
    (_, x1, x2, y1, y2): Tuple<String, "=", i32, "..", i32, ", y=", i32, "..", i32>,
) -> (i32, i32) {
    let mut res = 0;
    let mut count = 0;
    for init_vx in 1..=x2 {
        for init_vy in y1..=-y1 {
            let mut vx = init_vx;
            let mut vy = init_vy;
            let mut x = 0;
            let mut y = 0;
            let mut max_y = 0;
            let mut ok = false;
            while x <= x2 && y >= y1 {
                if x1 <= x && x <= x2 && y1 <= y && y <= y2 {
                    ok = true;
                }
                x += vx;
                y += vy;
                max_y = max_y.max(y);
                vx = 0.max(vx - 1);
                vy = vy - 1;
            }
            if ok {
                res = res.max(max_y);
                count += 1;
            }
        }
    }
    (res, count)
}

aoc_test!(2021, 17, "target area: x=20..30, y=-10..-5", 45, 112);
