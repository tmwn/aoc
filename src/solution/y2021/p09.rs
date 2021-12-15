use aocio::aocio;

use crate::solution::util;

#[aocio]
pub fn small(cave: Vec<Vec<i32, "">>) -> i32 {
    let h = cave.len();
    let w = cave[0].len();

    let mut res = 0;
    for i in 0..h {
        for j in 0..w {
            if low_point(&cave, i, j) {
                res += cave[i][j] + 1;
            }
        }
    }
    res
}

#[aocio]
pub fn large(cave: Vec<Vec<i32, "">>) -> i32 {
    let h = cave.len();
    let w = cave[0].len();

    let mut count = vec![vec![0; w]; h];
    for i in 0..h {
        for j in 0..w {
            if cave[i][j] == 9 {
                continue;
            }
            let (x, y) = sink(&cave, i, j);
            count[x][y] += 1;
        }
    }
    let mut a = vec![];
    for cs in count {
        for c in cs {
            if c > 0 {
                a.push(c);
            }
        }
    }
    a.sort_unstable();
    let mut res = 1;
    for i in 1..4 {
        res *= a[a.len() - i];
    }
    res
}

fn sink(cave: &[Vec<i32>], mut i: usize, mut j: usize) -> (usize, usize) {
    while !low_point(cave, i, j) {
        for (x, y) in util::grid::neighbors(i, j, cave.len(), cave[0].len()) {
            if cave[x][y] < cave[i][j] {
                i = x;
                j = y;
            }
        }
    }
    (i, j)
}

fn low_point(cave: &[Vec<i32>], i: usize, j: usize) -> bool {
    let cur = cave[i][j];
    for (x, y) in util::grid::neighbors(i, j, cave.len(), cave[0].len()) {
        if cur >= cave[x][y] {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::super::solve;

    const INPUT: &str = r#"2199943210
3987894921
9856789892
8767896789
9899965678"#;

    #[test]
    fn small() {
        assert_eq!(solve(INPUT, 9, false).unwrap(), "15");
    }
    #[test]
    fn large() {
        assert_eq!(solve(INPUT, 9, true).unwrap(), "1134");
    }
}
