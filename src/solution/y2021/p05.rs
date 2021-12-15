use std::str::FromStr;

use aocio::aocio;

#[aocio]
pub fn small(ss: Vec<Segment>) -> i32 {
    solve(ss, false)
}

#[aocio]
pub fn large(ss: Vec<Segment>) -> i32 {
    solve(ss, true)
}

fn solve(ss: Vec<Segment>, use_diagonal: bool) -> i32 {
    let mut count = vec![vec![0; 1000]; 1000];
    for s in ss.iter() {
        if !use_diagonal && s.slant() {
            continue;
        }
        for p in s.points() {
            count[p.x as usize][p.y as usize] += 1;
        }
    }
    let mut res = 0;
    for cs in count {
        for c in cs {
            if c >= 2 {
                res += 1;
            }
        }
    }
    res
}

#[derive(Debug)]
pub struct Segment {
    p1: Point,
    p2: Point,
}

impl Segment {
    fn points(&self) -> Vec<Point> {
        let mut n = 0;
        n = n.max((self.p2.x - self.p1.x).abs());
        n = n.max((self.p2.y - self.p1.y).abs());

        let dx = (self.p2.x - self.p1.x) / n;
        let dy = (self.p2.y - self.p1.y) / n;

        let mut res = vec![];
        for i in 0..=n {
            let x = self.p1.x + dx * i;
            let y = self.p1.y + dy * i;
            res.push(Point { x, y });
        }
        res
    }

    fn slant(&self) -> bool {
        self.p1.x != self.p2.x && self.p1.y != self.p2.y
    }
}

impl FromStr for Segment {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ss = s.split(" -> ").collect::<Vec<_>>();
        let p1 = ss[0].parse()?;
        let p2 = ss[1].parse()?;
        Ok(Segment { p1, p2 })
    }
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl FromStr for Point {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ss = s.split(',').collect::<Vec<_>>();
        let x = ss[0].parse()?;
        let y = ss[1].parse()?;
        Ok(Point { x, y })
    }
}

#[cfg(test)]
mod tests {
    use super::super::solve;

    #[test]
    fn small() {
        assert_eq!(
            solve(
                r#"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"#,
                5,
                false,
            )
            .unwrap(),
            "5"
        );
    }
}
