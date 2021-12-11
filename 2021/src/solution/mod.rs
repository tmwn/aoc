mod util;

mod p01;
mod p02;
mod p03;
mod p04;
mod p05;
mod p06;
mod p07;
mod p08;
mod p09;
mod p10;
mod p11;
mod p12;
mod p13;
mod p14;
mod p15;
mod p16;
mod p17;
mod p18;
mod p19;
mod p20;
mod p21;
mod p22;
mod p23;
mod p24;
mod p25;

pub fn solve(input: &str, day: i32, large: bool) -> anyhow::Result<String> {
    Ok(match (large, day) {
        (false, 1) => p01::small(parse(input)).to_string(),
        (false, 2) => p02::small(parse(input)).to_string(),
        (false, 3) => p03::small(parse(input)).to_string(),
        (false, 4) => p04::small(parse(input)).to_string(),
        (false, 5) => p05::small(parse(input)).to_string(),
        (false, 6) => p06::small(parse(input)).to_string(),
        (false, 7) => p07::small(parse(input)).to_string(),
        (false, 8) => p08::small(parse(input)).to_string(),
        (false, 9) => p09::small(parse(input)).to_string(),
        (false, 10) => p10::small(parse(input)).to_string(),
        (false, 11) => p11::small(parse(input)).to_string(),
        (false, 12) => p12::small(parse(input)).to_string(),
        (false, 13) => p13::small(parse(input)).to_string(),
        (false, 14) => p14::small(parse(input)).to_string(),
        (false, 15) => p15::small(parse(input)).to_string(),
        (false, 16) => p16::small(parse(input)).to_string(),
        (false, 17) => p17::small(parse(input)).to_string(),
        (false, 18) => p18::small(parse(input)).to_string(),
        (false, 19) => p19::small(parse(input)).to_string(),
        (false, 20) => p20::small(parse(input)).to_string(),
        (false, 21) => p21::small(parse(input)).to_string(),
        (false, 22) => p22::small(parse(input)).to_string(),
        (false, 23) => p23::small(parse(input)).to_string(),
        (false, 24) => p24::small(parse(input)).to_string(),
        (false, 25) => p25::small(parse(input)).to_string(),
        (true, 1) => p01::large(parse(input)).to_string(),
        (true, 2) => p02::large(parse(input)).to_string(),
        (true, 3) => p03::large(parse(input)).to_string(),
        (true, 4) => p04::large(parse(input)).to_string(),
        (true, 5) => p05::large(parse(input)).to_string(),
        (true, 6) => p06::large(parse(input)).to_string(),
        (true, 7) => p07::large(parse(input)).to_string(),
        (true, 8) => p08::large(parse(input)).to_string(),
        (true, 9) => p09::large(parse(input)).to_string(),
        (true, 10) => p10::large(parse(input)).to_string(),
        (true, 11) => p11::large(parse(input)).to_string(),
        (true, 12) => p12::large(parse(input)).to_string(),
        (true, 13) => p13::large(parse(input)).to_string(),
        (true, 14) => p14::large(parse(input)).to_string(),
        (true, 15) => p15::large(parse(input)).to_string(),
        (true, 16) => p16::large(parse(input)).to_string(),
        (true, 17) => p17::large(parse(input)).to_string(),
        (true, 18) => p18::large(parse(input)).to_string(),
        (true, 19) => p19::large(parse(input)).to_string(),
        (true, 20) => p20::large(parse(input)).to_string(),
        (true, 21) => p21::large(parse(input)).to_string(),
        (true, 22) => p22::large(parse(input)).to_string(),
        (true, 23) => p23::large(parse(input)).to_string(),
        (true, 24) => p24::large(parse(input)).to_string(),
        (true, 25) => p25::large(parse(input)).to_string(),
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
        s.split('\n').map(|s| s.parse::<S>().unwrap()).collect()
    }
}

impl Parse for String {
    fn parse(s: &str) -> Self {
        s.to_string()
    }
}
