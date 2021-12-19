pub const DIR4: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

pub const DIR8: [(isize, isize); 8] = [
    (1, 0),
    (0, 1),
    (-1, 0),
    (0, -1),
    (1, 1),
    (-1, 1),
    (-1, -1),
    (1, -1),
];

pub(crate) fn neighbors(
    x: usize,
    y: usize,
    h: usize,
    w: usize,
) -> impl std::iter::Iterator<Item = (usize, usize)> {
    DIR4.iter().filter_map(move |d| {
        let x = x as isize + d.0;
        let y = y as isize + d.1;
        if x >= 0 && y >= 0 && x < h as isize && y < w as isize {
            Some((x as usize, y as usize))
        } else {
            None
        }
    })
}

pub(crate) fn neighbors8(
    x: usize,
    y: usize,
    h: usize,
    w: usize,
) -> impl std::iter::Iterator<Item = (usize, usize)> {
    DIR8.iter().filter_map(move |d| {
        let x = x as isize + d.0;
        let y = y as isize + d.1;
        if x >= 0 && y >= 0 && x < h as isize && y < w as isize {
            Some((x as usize, y as usize))
        } else {
            None
        }
    })
}
