use super::Parse;

pub fn small(input: Input) -> i64 {
    minimize(|m| {
        let mut res = 0;
        for x in input.0.iter() {
            res += (m - x).abs() as i64
        }
        res
    })
}

pub fn large(input: Input) -> i64 {
    minimize(|m| {
        let mut res = 0;
        for x in input.0.iter() {
            let d = (m - x).abs() as i64;
            res += d * (d + 1) / 2;
        }
        res
    })
}

pub struct Input(Vec<i32>);

impl Parse for Input {
    fn parse(s: &str) -> Self {
        Input(
            s.trim()
                .split(",")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>(),
        )
    }
}

fn minimize<F>(f: F) -> i64
where
    F: Fn(i32) -> i64,
{
    let mut left = 0;
    let mut right = 1000000;
    while left + 1 != right {
        let m1 = (left * 2 + right + 1) / 3;
        let m2 = (left + right * 2) / 3;
        if f(m1) < f(m2) {
            right = m2;
        } else {
            left = m1;
        }
    }
    let lv = f(left);
    let rv = f(right);
    lv.min(rv)
}

#[cfg(test)]
mod tests {
    use crate::solution::solve;

    const INPUT: &'static str = r#"16,1,2,0,4,2,7,1,2,14
"#;
    #[test]
    fn small() {
        assert_eq!(solve(INPUT, 7, false).unwrap(), "37");
    }
    #[test]
    fn large() {
        assert_eq!(solve(INPUT, 7, true).unwrap(), "168");
    }
}
