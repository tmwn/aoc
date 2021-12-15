use std::ops::{Deref, DerefMut};

use crate::solution::Parse;

pub const DIR4: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

pub const DIR8: [(isize, isize); 8] = [
    (0, 1),
    (1, 0),
    (0, -1),
    (-1, 0),
    (-1, -1),
    (-1, 1),
    (1, -1),
    (1, 1),
];

pub struct Grid<S>(Vec<Vec<S>>);

impl<S> Deref for Grid<S> {
    type Target = Vec<Vec<S>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<S> DerefMut for Grid<S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Parse for Grid<i32> {
    fn parse(s: &str) -> Self {
        Self(
            s.trim()
                .split('\n')
                .map(|s| s.bytes().map(|x| (x - b'0') as i32).collect())
                .collect(),
        )
    }
}

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
