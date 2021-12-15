use aocio::aocio;

use crate::solution::aoc_test;

#[aocio]
pub fn small(
    (order, cards): Tuple<Vec<usize, ",">, "\n\n", Vec<Vec<Vec<usize, " ">>, "\n\n">>,
) -> usize {
    let mut cards: Vec<_> = cards.into_iter().map(|card| Card::new(card)).collect();
    for o in order.iter() {
        for c in cards.iter_mut() {
            c.set(*o);
            if c.bingo() {
                return c.score() * o;
            }
        }
    }
    panic!("should not reach")
}

#[aocio]
pub fn large(
    (order, cards): Tuple<Vec<usize, ",">, "\n\n", Vec<Vec<Vec<usize, " ">>, "\n\n">>,
) -> usize {
    let mut cards: Vec<_> = cards.into_iter().map(|card| Card::new(card)).collect();

    let n = cards.len();
    let mut count = 0;
    let mut done = vec![false; n];
    for o in order.iter() {
        for (i, c) in cards.iter_mut().enumerate() {
            if done[i] {
                continue;
            }
            c.set(*o);
            if c.bingo() {
                done[i] = true;
                count += 1;
                if count == n {
                    return c.score() * o;
                }
            }
        }
    }
    panic!("shoud not reach")
}

type Board<S> = [[S; 5]; 5];

#[derive(Default)]
struct Card {
    board: Vec<Vec<usize>>,
    rev: Vec<Option<(usize, usize)>>,
    called: Board<bool>,
}

impl Card {
    fn new(board: Vec<Vec<usize>>) -> Card {
        let mut rev = vec![None; 100];
        for i in 0..5 {
            for j in 0..5 {
                rev[board[i][j]] = Some((i, j));
            }
        }
        let called = Board::default();
        Card { board, rev, called }
    }
    fn set(&mut self, i: usize) {
        if let Some((r, c)) = self.rev[i] {
            self.called[r][c] = true;
        }
    }
    fn bingo(&self) -> bool {
        for i in 0..5 {
            let mut ok = true;
            for j in 0..5 {
                if !self.called[i][j] {
                    ok = false;
                }
            }
            if ok {
                return true;
            }
        }
        for i in 0..5 {
            let mut ok = true;
            for j in 0..5 {
                if !self.called[j][i] {
                    ok = false;
                }
            }
            if ok {
                return true;
            }
        }
        false
    }
    fn score(&self) -> usize {
        let mut res = 0;
        for i in 0..5 {
            for j in 0..5 {
                if !self.called[i][j] {
                    res += self.board[i][j];
                }
            }
        }
        res
    }
}

aoc_test!(
    2021,
    4,
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
 2  0 12  3  7",
    4512,
    1924
);
