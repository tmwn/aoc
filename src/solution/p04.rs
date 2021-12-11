use std::str::FromStr;

pub fn small(a: String) -> usize {
    let Problem { order, mut cards } = a.parse().unwrap();

    for o in order {
        for c in cards.iter_mut() {
            c.set(o);
            if c.bingo() {
                return c.score() * o;
            }
        }
    }
    panic!("should not reach")
}

pub fn large(a: String) -> i32 {
    todo!()
}

struct Problem {
    order: Vec<usize>,
    cards: Vec<Card>,
}

impl FromStr for Problem {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split_ascii_whitespace();
        let order = it
            .next()
            .unwrap()
            .split(",")
            .map(|s| s.parse().unwrap())
            .collect();
        let mut cards = vec![];
        while let Some(first) = it.next() {
            let mut board = Board::default();
            board[0][0] = first.parse()?;
            for i in 1..25 {
                board[i / 5][i % 5] = it.next().unwrap().parse().unwrap();
            }
            cards.push(Card::new(board));
        }
        Ok(Self { order, cards })
    }
}

type Board<S> = [[S; 5]; 5];

#[derive(Default)]
struct Card {
    board: Board<usize>,
    rev: Vec<Option<(usize, usize)>>,
    called: Board<bool>,
}

impl Card {
    fn new(board: Board<usize>) -> Card {
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
