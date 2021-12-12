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

gen_solve!(
    p01, p02, p03, p04, p05, p06, p07, p08, p09, p10, p11, p12, p13, p14, p15, p16, p17, p18, p19,
    p20, p21, p22, p23, p24, p25
);
