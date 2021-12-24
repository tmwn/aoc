use std::collections::{BTreeSet, BinaryHeap};

use aocio::aocio;

use crate::solution::aoc_test;

#[aocio]
pub fn small(a: Vec<String>) -> i32 {
    solve(a)
}

#[aocio]
pub fn large(mut a: Vec<String>) -> i32 {
    a.insert(3, "#D#C#B#A#".to_string());
    a.insert(4, "#D#B#A#C#".to_string());
    solve(a)
}

fn solve(mut a: Vec<String>) -> i32 {
    for i in 0..a.len() {
        while a[i].len() < a[0].len() {
            a[i] = " ".to_string() + &a[i] + " ";
        }
    }
    let room_height = a.len() - 3;
    let start_node = Node {
        score: 0,
        state: State::new(a),
    };
    let mut q = BinaryHeap::new();
    q.push(start_node);
    let mut seen = BTreeSet::new();
    while let Some(node) = q.pop() {
        if node.state.is_goal(room_height) {
            return -node.score;
        }
        if seen.contains(&node.state) {
            continue;
        }
        seen.insert(node.state.clone());

        for (delta, next_state) in node.state.next_states(room_height) {
            let n_score = node.score - delta;
            q.push(Node {
                score: n_score,
                state: next_state,
            });
        }
    }
    panic!("not found")
}

type Mask = i32;

const COST: [i32; 4] = [1, 10, 100, 1000];

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Node {
    score: i32,
    state: State,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct State {
    hallway: Vec<Option<u8>>,
    hallway_mask: Mask,
    rooms: Vec<Vec<u8>>, // stacks
}

fn mask_between(mut i: usize, mut j: usize) -> Mask {
    if i == j {
        return 0;
    }
    if i > j {
        std::mem::swap(&mut i, &mut j);
    }
    (1 << j) - (1 << (i + 1))
}

impl State {
    fn new(a: Vec<String>) -> State {
        let room_height = a.len() - 3;
        let mut rooms = vec![vec![]; 4];
        for i in 0..4 {
            for j in 0..room_height {
                rooms[i].push(a[a.len() - 2 - j].as_bytes()[3 + i * 2] - b'A');
            }
        }
        State {
            hallway: vec![None; 11],
            hallway_mask: 0,
            rooms,
        }
    }
    fn next_states(&self, room_height: usize) -> Vec<(i32, State)> {
        let mut res = vec![];
        for y1 in [0, 1, 3, 5, 7, 9, 10] {
            if let Some(c) = self.hallway[y1] {
                // hallway to room
                let y2 = (2 * (c + 1)) as usize;
                if self.hallway_mask & mask_between(y1, y2) > 0 {
                    continue;
                }
                if !self.can_return(c) {
                    continue;
                }
                let mut s = self.clone();
                s.hallway[y1] = None;
                s.hallway_mask &= !(1 << y1);
                s.rooms[c as usize].push(c);
                let dist = (y1 as i32 - y2 as i32).abs()
                    + 1
                    + (room_height - s.rooms[c as usize].len()) as i32;
                res.push((dist * COST[c as usize], s));
            } else {
                // room to hallway
                for (i, room) in self.rooms.iter().enumerate() {
                    if room.is_empty() {
                        continue;
                    }
                    let y2 = 2 * (i + 1);
                    if self.hallway_mask & mask_between(y1, y2) > 0 {
                        continue;
                    }
                    if self.can_return(i as u8) {
                        continue;
                    }
                    let mut s = self.clone();
                    let c = s.rooms[i].pop().unwrap();
                    s.hallway[y1] = Some(c);
                    s.hallway_mask |= 1 << y1;
                    let dist =
                        (y1 as i32 - y2 as i32).abs() + (room_height - s.rooms[i].len()) as i32;
                    res.push((dist * COST[c as usize], s));
                }
            }
        }
        res
    }
    fn can_return(&self, c: u8) -> bool {
        self.rooms[c as usize].iter().all(|i| *i == c)
    }
    fn is_goal(&self, room_height: usize) -> bool {
        if self.rooms.iter().any(|room| room.len() < room_height) {
            return false;
        }
        (0..4).all(|i| self.can_return(i))
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
