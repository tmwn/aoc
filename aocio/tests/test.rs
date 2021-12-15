// use aocio::Parse;

// aocio::make_answer!();

// Example: Attribute with input
// #[aocio::aocio]
// fn solve2021_04(
//     (order, cards): Tuple<Vec<i32, ",">, "\n\n", Vec<Vec<Vec<i32, " ">>, "\n\n">>,
// ) -> i32 {
//     todo!()
// }

#[aocio::aocio]
fn empty_separate(x: Vec<i32, "">) -> usize {
    (x[0] + x[1]) as usize
}

#[aocio::aocio]
fn comma_separate(x: Vec<i32, ",">) -> usize {
    (x[0] + x[1]) as usize
}

#[aocio::aocio]
fn default_separate(x: Vec<i32>) -> usize {
    (x[0] + x[1]) as usize
}

#[aocio::aocio]
fn tuple((x, y): Tuple<i32, "2", i32>) -> usize {
    (x + y) as usize
}

#[aocio::aocio]
fn tuple3((x, y, z): Tuple<i32, "2", i32, "4", i32>) -> usize {
    (x + y + z) as usize
}

#[test]
fn test() {
    assert_eq!(empty_separate("123".parse().unwrap()), 3);
    assert_eq!(comma_separate("1,2,3".parse().unwrap()), 3);
    assert_eq!(
        default_separate(
            "1
    2
    3"
            .parse()
            .unwrap()
        ),
        3
    );
    assert_eq!(tuple("123".parse().unwrap()), 4);
}

#[test]
fn test_tuple3() {
    assert_eq!(tuple3("12345".parse().unwrap()), 9);
}

struct VV(Vec<i32>);
impl std::str::FromStr for VV {
    type Err = <i32 as std::str::FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(VV(s
            .split("")
            .filter(|x| !x.trim().is_empty())
            .map(|x| x.trim().parse())
            .collect::<Result<_, _>>()?))
    }
}
