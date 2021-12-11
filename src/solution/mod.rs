pub mod p01;
pub mod p02;
pub mod p03;

pub fn solve(input: &str, day: i32, large: bool) -> anyhow::Result<String> {
    Ok(match (large, day) {
        (false, 1) => p01::small(parse(input)).to_string(),
        (false, 2) => p02::small(parse(input)).to_string(),
        (false, 3) => p03::small(parse(input)).to_string(),
        (true, 1) => p01::large(parse(input)).to_string(),
        (true, 2) => p02::large(parse(input)).to_string(),
        (true, 3) => p03::large(parse(input)).to_string(),
        _ => anyhow::bail!("not found {} {}", day, large),
    })
}

fn parse<S: Parse>(s: &str) -> S {
    S::parse(s)
}

pub trait Parse {
    fn parse(s: &str) -> Self;
}

impl<S> Parse for Vec<S>
where
    S: std::str::FromStr,
    S::Err: std::fmt::Debug,
{
    fn parse(s: &str) -> Self {
        s.split("\n").map(|s| s.parse::<S>().unwrap()).collect()
    }
}
