#[aocio::aocio]
fn solve2021_04(
    (order, cards): Tuple<Vec<i32, ",">, "\n\n", Vec<Vec<Vec<i32, " ">, "\n">, "\n\n">>,
) -> (usize, usize, usize, usize) {
    println!("!! order = {:#?}", order);
    println!("!! cards = {:#?}", cards);
    (order.len(), cards.len(), cards[0].len(), cards[0][0].len())
}

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

#[test]
fn test_real_aoc_input() {
    assert_eq!(
        solve2021_04(
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

    22 13 17 11  0
     8  2 23  4 24
    21  9 14 16  7
     6 10  3 18  5
     1 12 20 15 19

     3 15  0  2 22
     9 18 13 17  5
    19  8  7 25 23
    20 11 10 24  4
    14 21 16 12  6

    14 21 17 24  4
    10 16 15  9 19
    18  8 23 26 20
    22 11 13  6  5
     2  0 12  3  7"
                .parse()
                .unwrap()
        ),
        (27, 3, 5, 5),
    );
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
