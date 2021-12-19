#[aocio::aocio]
fn tuple3((x, y, z): Tuple<i32, "2", i32, "4", i32>) -> usize {
    (x + y + z) as usize
}

#[aocio::aocio]
fn tuple_singleton(x: Tuple<usize>) -> usize {
    x
}

#[aocio::aocio]
fn tuple_dont_care(x: Tuple<_, "2", usize>) -> usize {
    x
}

#[aocio::aocio]
fn tuple_dont_care2(x: Tuple<_, "2", usize, "4", _>) -> usize {
    x
}

#[test]
fn test() {
    assert_eq!(tuple3("12345".parse().unwrap()), 9);
    assert_eq!(tuple_singleton("12345".parse().unwrap()), 12345);
    assert_eq!(tuple_dont_care("12345".parse().unwrap()), 345);
    assert_eq!(tuple_dont_care2("12345".parse().unwrap()), 3);
}
