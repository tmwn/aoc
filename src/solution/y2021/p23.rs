use std::collections::{BTreeSet, BinaryHeap};

use aocio::aocio;

use crate::solution::{aoc_test, util::grid};

#[aocio]
pub fn small(a: Vec<String>) -> i32 {
    solve(a, false)
}
#[aocio]
pub fn large(a: Vec<String>) -> i32 {
    solve(a, true)
}

pub fn solve(mut a: Vec<String>, large: bool) -> i32 {
    for i in 0..a.len() {
        while a[i].len() < a[0].len() {
            a[i] = " ".to_string() + &a[i] + " ";
        }
    }
    if large {
        a.insert(3, "  #D#C#B#A#  ".to_string());
        a.insert(4, "  #D#B#A#C#  ".to_string());
    }

    let mut pos = vec![];

    let h = a.len();
    let w = a[0].len();

    for i in 0..h {
        for j in 0..w {
            match a[i].as_bytes()[j] {
                b'#' | b' ' => (),
                _ => {
                    pos.push((i, j));
                }
            }
        }
    }
    pos.sort();

    let mut state = vec![-1; pos.len()];
    for i in 0..pos.len() {
        let c = a[pos[i].0].as_bytes()[pos[i].1];
        if c == b'.' {
            continue;
        }
        state[i] = (c - b'A') as i8;
    }
    let n = pos.len();
    let mut dist = vec![vec![i32::MAX / 2; n]; n];
    for i in 0..n {
        let (x, y) = pos[i];
        for (nx, ny) in grid::neighbors(x, y, h, w) {
            if let Ok(j) = pos.binary_search(&(nx, ny)) {
                dist[i][j] = 1;
            }
        }
        dist[i][i] = 0;
    }
    for k in 0..pos.len() {
        for i in 0..pos.len() {
            for j in 0..pos.len() {
                dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
            }
        }
    }
    let mut next = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                if dist[i][k] == 1 && dist[i][j] == dist[i][k] + dist[k][j] {
                    next[i][j] = k;
                }
            }
        }
    }

    let mut q = BinaryHeap::new();
    q.push(Node { state, score: 0 });

    let mut seen = BTreeSet::new();
    let weight = [1, 10, 100, 1000];

    while let Some(n) = q.pop() {
        if n.is_goal(&pos) {
            return -n.score;
        }
        if seen.contains(&n.state) {
            continue;
        }
        seen.insert(n.state.clone());

        for i in 0..pos.len() {
            if n.state[i] == -1 {
                continue;
            }
            if n.is_goal_pos(i, &pos) {
                continue;
            }
            let (x1, y1) = pos[i];
            for j in 0..pos.len() {
                if n.state[j] != -1 {
                    continue;
                }
                let (x2, y2) = pos[j];
                if !n.is_ok(&pos, n.state[i], x2, y2, x1) {
                    continue;
                }
                let mut p = i;
                let mut ok = true;
                while p != j {
                    p = next[p][j];
                    if n.state[p] != -1 {
                        ok = false;
                        break;
                    }
                }
                if !ok {
                    continue;
                }

                let mut n_state = n.state.clone();
                n_state[j] = n.state[i];
                n_state[i] = -1;
                let n_score = n.score
                    - ((x1 as i32 - x2 as i32).abs() + (y1 as i32 - y2 as i32).abs())
                        * weight[n.state[i] as usize];
                let node = Node {
                    state: n_state,
                    score: n_score,
                };
                q.push(node);
            }
        }
    }
    panic!()
}

#[derive(PartialEq, Eq)]
struct Node {
    state: Vec<i8>,
    score: i32,
}

impl Node {
    fn is_goal(&self, pos: &Vec<(usize, usize)>) -> bool {
        for i in 0..pos.len() {
            let c = self.state[i];
            let want = match pos[i] {
                (1, _) => -1,
                (_, 3) => 0,
                (_, 5) => 1,
                (_, 7) => 2,
                (_, 9) => 3,
                _ => panic!(),
            };
            if c != want {
                return false;
            }
        }
        true
    }

    fn is_goal_pos(&self, i: usize, pos: &Vec<(usize, usize)>) -> bool {
        if self.state[i] == -1 {
            return false;
        }
        let c = self.state[i];
        let (x, y) = pos[i];
        if x == 1 {
            return false;
        }
        if c as usize != (y - 3) / 2 {
            return false;
        }
        for j in (x + 1).. {
            if let Ok(p) = pos.binary_search(&(j, y)) {
                if self.state[p] != c {
                    return false;
                }
            } else {
                return true;
            }
        }
        panic!()
    }

    fn is_ok(&self, pos: &Vec<(usize, usize)>, c: i8, x: usize, y: usize, from_x: usize) -> bool {
        if x == 1 {
            if from_x == 1 {
                return false;
            }
            if y == 3 || y == 5 || y == 7 || y == 9 {
                return false;
            }
            return true;
        }
        if from_x != 1 {
            return false;
        }
        if c as usize != (y - 3) / 2 {
            return false;
        }
        for j in (x + 1).. {
            if let Ok(p) = pos.binary_search(&(j, y)) {
                if self.state[p] != c {
                    return false;
                }
            } else {
                return true;
            }
        }
        panic!()
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.score.partial_cmp(&other.score)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score)
    }
}

aoc_test!(
    2021,
    23,
    "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########",
    12521,
    44169
);
