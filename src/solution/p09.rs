use super::Parse;

const DX: [isize; 4] = [0, 1, 0, -1];
const DY: [isize; 4] = [1, 0, -1, 0];

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

pub struct Cave(Vec<Vec<i32>>);

impl Cave {
    fn get(&self, i: isize, j: isize) -> Option<&i32> {
        if let Some(row) = self.0.get(i as usize) {
            row.get(j as usize)
        } else {
            None
        }
    }

    fn sink(&self, mut i: usize, mut j: usize) -> (usize, usize) {
        while !self.low_point(i, j) {
            for d in 0..4 {
                let ni = i as isize + DX[d];
                let nj = j as isize + DY[d];
                if let Some(adj) = self.get(ni, nj) {
                    if *adj < self.0[i][j] {
                        i = ni as usize;
                        j = nj as usize;
                    }
                }
            }
        }
        (i, j)
    }

    fn low_point(&self, i: usize, j: usize) -> bool {
        let cur = self.0[i][j];
        for d in 0..4 {
            if let Some(adj) = self.get(i as isize + DX[d], j as isize + DY[d]) {
                if cur >= *adj {
                    return false;
                }
            }
        }
        true
    }
}

impl Parse for Cave {
    fn parse(s: &str) -> Self {
        Cave(
            s.trim()
                .split('\n')
                .map(|s| s.bytes().map(|x| (x - b'0') as i32).collect())
                .collect(),
        )
    }
}
