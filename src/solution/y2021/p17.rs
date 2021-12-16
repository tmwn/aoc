use aocio::aocio;

pub fn small(s: String) -> i32 {
    solve(s.parse().unwrap()).0
}

pub fn large(s: String) -> i32 {
    solve(s.parse().unwrap()).1
}

#[aocio]
fn solve(_: Vec<String>) -> (i32, i32) {
    todo!()
}

// aoc_test!(2021, 17, "", 0, 0);
