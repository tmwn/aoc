pub fn small(a: Vec<String>) -> i32 {
    let mut v = vec![0; a[0].len()];
    for s in a.iter() {
        for (i, c) in s.chars().enumerate() {
            if c == '1' {
                v[i] += 1;
            }
        }
    }
    let mut g = 0;
    let mut e = 0;
    for (i, k) in v.iter().enumerate() {
        if k * 2 > a.len() {
            g |= 1 << (v.len() - 1 - i);
        } else {
            e |= 1 << (v.len() - 1 - i);
        }
    }
    g * e
}

pub fn large(a: Vec<String>) -> i32 {
    let n = a[0].len();
    let mut gs = a.clone();
    let mut es = a;

    for i in 0..n {
        gs = take(gs, i, true);
        es = take(es, i, false);
    }
    i32::from_str_radix(&gs[0], 2).unwrap() * i32::from_str_radix(&es[0], 2).unwrap()
}

fn take(a: Vec<String>, i: usize, majority: bool) -> Vec<String> {
    if a.len() == 1 {
        return a;
    }
    let mut c = 0;
    for s in a.iter() {
        if s.as_bytes()[i] == b'1' {
            c += 1;
        }
    }
    let more = if 2 * c >= a.len() { b'1' } else { b'0' };
    a.into_iter()
        .filter(|s| (s.as_bytes()[i] == more) == majority)
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::solution::solve;

    #[test]
    fn large() {
        assert_eq!(
            solve(
                r#"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"#,
                3,
                true,
            )
            .unwrap(),
            "230"
        );
    }
}
