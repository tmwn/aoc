use aocio::aocio;
use itertools::Itertools;

use crate::solution::aoc_test;

#[aocio]
pub fn small(s: String) -> u64 {
    solve(s).0
}

#[aocio]
pub fn large(s: String) -> u128 {
    solve(s).1
}

pub fn solve(s: String) -> (u64, u128) {
    let input = s
        .bytes()
        .map(|x| {
            let n = if (b'A'..=b'F').contains(&x) {
                x - b'A' + 10
            } else {
                x - b'0'
            };
            format!("{:04b}", n)
        })
        .join("");
    let packet = parse(&input);
    (packet.version_sum(), packet.compute())
}

fn parse_num(s: &str, cur: u128) -> (u128, usize) {
    let x = &s[0..5];
    let n = u128::from_str_radix(&s[1..5], 2).unwrap();
    if x.as_bytes()[0] == b'0' {
        (cur * 16 + n, 1)
    } else {
        let (x, y) = parse_num(&s[5..], cur * 16 + n);
        (x, y + 1)
    }
}

fn parse(s: &str) -> Packet {
    let version = u8::from_str_radix(&s[0..3], 2).unwrap();
    let type_id = u8::from_str_radix(&s[3..6], 2).unwrap();
    if type_id == 4 {
        let (value, d) = parse_num(&s[6..], 0);
        Packet {
            sub: vec![],
            type_id,
            version,
            value: Some(value),
            len: d * 5 + 6,
            num: 1,
        }
    } else {
        let length_type_id = s[6..7].as_bytes()[0] - b'0';
        if length_type_id == 0 {
            let total_len = usize::from_str_radix(&s[7..22], 2).unwrap();
            let mut len = 0;
            let mut sub = vec![];
            while len != total_len {
                let p = parse(&s[len + 22..]);
                len += p.len;
                sub.push(p);
            }
            Packet {
                sub,
                type_id,
                version,
                value: None,
                len: len + 22,
                num: 1,
            }
        } else {
            let total_num = usize::from_str_radix(&s[7..18], 2).unwrap();
            let mut num = 0;
            let mut len = 0;
            let mut sub = vec![];
            while num != total_num {
                let p = parse(&s[len + 18..]);
                num += p.num;
                len += p.len;
                sub.push(p);
            }
            Packet {
                sub,
                type_id,
                version,
                value: None,
                len: len + 18,
                num: 1,
            }
        }
    }
}

#[derive(Debug)]
struct Packet {
    sub: Vec<Packet>,
    type_id: u8,
    version: u8,
    value: Option<u128>,
    len: usize,
    num: usize,
}

impl Packet {
    fn version_sum(&self) -> u64 {
        let mut res = self.version as u64;
        for s in self.sub.iter() {
            res += s.version_sum()
        }
        res
    }
    fn compute(&self) -> u128 {
        let sub: Vec<_> = self.sub.iter().map(|p| p.compute()).collect();
        match self.type_id {
            0 => sub.iter().sum(),
            1 => sub.iter().product(),
            2 => *sub.iter().min().unwrap(),
            3 => *sub.iter().max().unwrap(),
            4 => self.value.unwrap(),
            5 => {
                if sub[0] > sub[1] {
                    1
                } else {
                    0
                }
            }
            6 => {
                if sub[0] < sub[1] {
                    1
                } else {
                    0
                }
            }
            7 => {
                if sub[0] == sub[1] {
                    1
                } else {
                    0
                }
            }
            _ => panic!(),
        }
    }
}

aoc_test!(2021, 16, "8A004A801A8002F478", 16, , s1);
aoc_test!(2021, 16, "620080001611562C8802118E34", 12, , s2);
aoc_test!(2021, 16, "C0015000016115A2E0802F182340", 23, , s3);
aoc_test!(2021, 16, "A0016C880162017C3686B18A3D4780", 31, , s4);

aoc_test!(2021, 16, "C200B40A82", , 3, tests2);
aoc_test!(2021, 16, "04005AC33890", , 54, tests3);
aoc_test!(2021, 16, "880086C3E88112", , 7, tests4);
aoc_test!(2021, 16, "CE00C43D881120", , 9, tests5);
aoc_test!(2021, 16, "D8005AC2A8F0", , 1, tests6);
aoc_test!(2021, 16, "F600BC2D8F", , 0, tests7);
aoc_test!(2021, 16, "9C005AC2F8F0", , 0, tests8);
aoc_test!(2021, 16, "9C0141080250320F1802104A08", , 1, tests9);
