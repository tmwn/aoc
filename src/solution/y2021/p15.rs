use std::collections::BTreeSet;

use crate::solution::{
    aoc_test,
    util::{self, grid::Grid},
};

pub fn small(grid: Grid<i32>) -> i32 {
    let h = grid.len();
    let w = grid[0].len();
    let mut visited = vec![vec![false; w]; h];

    let mut q = std::collections::BinaryHeap::<(i32, usize, usize)>::new();
    q.push((0, 0, 0));

    while let Some((d, x, y)) = q.pop() {
        if visited[x][y] {
            continue;
        }
        visited[x][y] = true;
        if x == h - 1 && y == w - 1 {
            return -d;
        }
        for (nx, ny) in util::grid::neighbors(x, y, h, w) {
            q.push((d - grid[nx][ny], nx, ny));
        }
    }
    panic!()
}

pub fn large(grid: Grid<i32>) -> i32 {
    let h = grid.len();
    let w = grid[0].len();
    let mut visited = BTreeSet::new();

    let mut q = std::collections::BinaryHeap::<(i32, usize, usize)>::new();
    q.push((0, 0, 0));

    while let Some((d, x, y)) = q.pop() {
        if visited.contains(&(x, y)) {
            continue;
        }
        visited.insert((x, y));
        if x == h * 5 - 1 && y == w * 5 - 1 {
            return -d;
        }
        for (x, y) in util::grid::neighbors(x, y, 5 * h, 5 * w) {
            let v = (grid[x % h][y % w] as usize + x / h + y / w - 1) % 9 + 1;
            q.push((d - v as i32, x, y));
        }
    }
    panic!()
}

aoc_test!(
    2021,
    15,
    "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581",
    40,
    315
);
