use std::collections::HashMap;

mod util;

macro_rules! years {
    (
        $($year: expr, $id:ident);*
    ) => {
        $(
            mod $id;
        )*

        pub fn solve(input: &str, year: i32, day: i32, large: bool) -> anyhow::Result<String> {
            type Solve = Box<dyn Fn(&str, i32, bool) -> anyhow::Result<String>>;
            let mut solvers: HashMap<i32, Solve> = HashMap::new();
            $(
                solvers.insert($year, Box::new($id::solve));
            )*

            solvers
                .get(&year)
                .ok_or_else(|| anyhow::anyhow!("not found {}", year))?(input, day, large)
        }
    }
}

years!(2020, y2020; 2021, y2021);

macro_rules! days {
    (
        $($id:ident),*
    ) => {
        $(
            mod $id;
        )*
        type Solve = Box<dyn Fn(&str)->String>;
        pub (crate) fn solve(input: &str, day: i32, large: bool) -> anyhow::Result<String> {
            let mut solvers: Vec<(Solve, Solve)> = vec![];
            $(
                solvers.push((
                    Box::new(|input|$id::small(super::parse(input)).to_string()),
                    Box::new(|input|$id::large(super::parse(input)).to_string()),
                ));
            )*

            Ok(match large {
                false => solvers[(day-1) as usize].0(input),
                true => solvers[(day-1) as usize].1(input),
            })
        }
    };
}
pub(crate) use days;

pub fn parse<S: Parse>(s: &str) -> S {
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn error() {
        assert_eq!(solve("", 0, 0, false).is_err(), true);
    }
}
