mod util;

macro_rules! gen_solve {
    (
        $($id:ident),*
    ) => {
        $(
            mod $id;
        )*
        type Solve = Box<dyn Fn(&str)->String>;
        pub fn solve(input: &str, day: i32, large: bool) -> anyhow::Result<String> {
            let mut solvers: Vec<(Solve, Solve)> = vec![];
            $(
                solvers.push((
                    Box::new(|input|$id::small(parse(input)).to_string()),
                    Box::new(|input|$id::large(parse(input)).to_string()),
                ));
            )*

            Ok(match large {
                false => solvers[(day-1) as usize].0(input),
                true => solvers[(day-1) as usize].1(input),
            })
        }
    };
}

gen_solve!(
    p01, p02, p03, p04, p05, p06, p07, p08, p09, p10, p11, p12, p13, p14, p15, p16, p17, p18, p19,
    p20, p21, p22, p23, p24, p25
);

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
