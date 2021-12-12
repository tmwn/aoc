use crate::solution::util::grid::Grid;

pub fn small(mut a: Grid<i32>) -> i32 {
    let mut res = 0;
    for _ in 0..100 {
        res += step(&mut a);
    }
    res
}

pub fn large(mut a: Grid<i32>) -> i32 {
    for i in 1.. {
        if step(&mut a) == 100 {
            return i;
        }
    }
    panic!()
}

fn step(a: &mut Grid<i32>) -> i32 {
    for i in 0..10 {
        for j in 0..10 {
            increment(a, i, j);
        }
    }
    let mut res = 0;
    for i in 0..10 {
        for j in 0..10 {
            if a[i][j] >= 10 {
                res += 1;
                a[i][j] = 0;
            }
        }
    }
    res
}

fn increment(a: &mut Grid<i32>, i: usize, j: usize) {
    a[i][j] += 1;
    if a[i][j] != 10 {
        return;
    }
    let ds: Vec<(usize, usize)> = a
        .enumerate_adjecent(i, j, true)
        .map(|(i, j, _)| (i, j))
        .collect();
    for (i, j) in ds {
        increment(a, i, j)
    }
}

#[cfg(test)]
mod tests {
    use super::super::solve;

    const INPUT: &str = r#"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"#;

    #[test]
    fn small() {
        assert_eq!(solve(INPUT, 11, false).unwrap(), "1656");
    }
    #[test]
    fn large() {
        assert_eq!(solve(INPUT, 11, true).unwrap(), "195");
    }
}
