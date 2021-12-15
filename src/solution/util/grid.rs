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

impl<S> Grid<S> {
    pub fn get(&self, i: isize, j: isize) -> Option<&S> {
        self.0.get(i as usize).and_then(|row| row.get(j as usize))
    }

    pub fn iter_adjecent(
        &self,
        i: usize,
        j: usize,
        diagonal: bool,
    ) -> impl std::iter::Iterator<Item = &S> {
        self.enumerate_adjecent(i, j, diagonal).map(|(_, _, v)| v)
    }

    pub fn enumerate_adjecent(
        &self,
        i: usize,
        j: usize,
        diagonal: bool,
    ) -> impl std::iter::Iterator<Item = (usize, usize, &S)> {
        let ds = DIR8.iter().take(if diagonal { 8 } else { 4 });
        ds.filter_map(move |d| {
            let x = i as isize + d.0;
            let y = j as isize + d.1;
            self.get(x, y).map(|v| (x as usize, y as usize, v))
        })
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
