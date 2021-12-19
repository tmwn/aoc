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

years!(2019, y2019; 2020, y2020; 2021, y2021);

macro_rules! days {
    (
        $($id:ident),*
    ) => {
        $(
            mod $id;
        )*
        type Solve = Box<dyn Fn(&str)->String>;
        pub (crate) fn solve(input: &str, day: i32, large: bool) -> anyhow::Result<String> {
            let solvers: Vec<(Solve, Solve)> = vec![
                $(
                (
                    Box::new(|input|$id::small(input.parse().unwrap()).to_string()),
                    Box::new(|input|$id::large(input.parse().unwrap()).to_string()),
                ),
                )*
            ];

            Ok(match large {
                false => solvers[(day-1) as usize].0(input),
                true => solvers[(day-1) as usize].1(input),
            })
        }
    };
}
pub(crate) use days;

macro_rules! aoc_test {
    ($year: literal, $day: literal, $input: literal, $($small: literal)? $(,$large: literal)?) => {
        crate::solution::aoc_test!($year, $day, $input, $($small)?, $($large)?, tests);
    };
    ($year: literal, $day: literal, $input: literal, $($small: literal)?, $($large: literal)?, $module_name: ident) => {
        #[cfg(test)]
        mod $module_name {
            const INPUT: &str = $input;
            $(
                #[test]
                fn small() {
                assert_eq!(
                    crate::solution::solve(INPUT, $year, $day, false).unwrap(),
                    $small.to_string()
                );
            })?
            $(
                #[test]
                fn large() {
                    assert_eq!(
                        crate::solution::solve(INPUT, $year, $day, true).unwrap(),
                        $large.to_string()
                    );
                }
            )?
        }
    };

}
pub(crate) use aoc_test;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn error() {
        assert!(solve("", 0, 0, false).is_err());
    }
}
