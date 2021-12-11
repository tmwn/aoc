use super::{util::grid::Grid, Parse};

pub fn small(cave: Cave) -> i32 {
    let h = cave.0.len();
    let w = cave.0[0].len();

    let mut res = 0;
    for i in 0..h {
        for j in 0..w {
            if cave.low_point(i, j) {
                res += cave.0[i][j] + 1;
            }
        }
    }
    res
}

pub fn large(cave: Cave) -> i32 {
    let h = cave.0.len();
    let w = cave.0[0].len();

    let mut count = vec![vec![0; w]; h];
    for i in 0..h {
        for j in 0..w {
            if cave.0[i][j] == 9 {
                continue;
            }
            let (x, y) = cave.sink(i, j);
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

pub struct Cave(Grid<i32>);

impl Cave {
    fn sink(&self, mut i: usize, mut j: usize) -> (usize, usize) {
        while !self.low_point(i, j) {
            for (x, y, v) in self.0.enumerate_adjecent(i, j, false) {
                if *v < self.0[i][j] {
                    i = x;
                    j = y;
                }
            }
        }
        (i, j)
    }

    fn low_point(&self, i: usize, j: usize) -> bool {
        let cur = self.0[i][j];
        for v in self.0.iter_adjecent(i, j, false) {
            if cur >= *v {
                return false;
            }
        }
        true
    }
}

impl Parse for Cave {
    fn parse(s: &str) -> Self {
        Cave(Grid::parse(s))
    }
}

#[cfg(test)]
mod tests {
    use crate::solution::solve;

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
