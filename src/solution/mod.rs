pub mod p01;

pub fn solve(input: &str, day: i32, large: bool) -> anyhow::Result<String> {
    Ok(match (large, day) {
        (false, 1) => p01::small(parse(input)).to_string(),
        (true, 1) => p01::large(parse(input)).to_string(),
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
