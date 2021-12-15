use aocio::aocio;

#[aocio]
pub fn small(input: Vec<i32, ",">) -> i64 {
    minimize(|m| {
        let mut res = 0;
        for x in input.iter() {
            res += (m - x).abs() as i64
        }
        res
    })
}

#[aocio]
pub fn large(input: Vec<i32, ",">) -> i64 {
    minimize(|m| {
        let mut res = 0;
        for x in input.iter() {
            let d = (m - x).abs() as i64;
            res += d * (d + 1) / 2;
        }
        res
    })
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
    use super::super::solve;

    const INPUT: &str = r#"16,1,2,0,4,2,7,1,2,14
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
