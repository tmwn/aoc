use aocio::aocio;

use crate::solution::aoc_test;

#[aocio]
pub fn small(mut p: Vec<Tuple<_, ": ", usize>>) -> usize {
    let mut score = [0, 0];

    let mut die = 1;
    let mut turn = 0;

    for i in 1.. {
        let mut m = 0;
        for _ in 0..3 {
            m += die;
            die += 1;
            if die > 100 {
                die = 1;
            }
        }
        p[turn % 2] = (p[turn % 2] + m) % 10;
        if p[turn % 2] == 0 {
            p[turn % 2] = 10;
        }
        score[turn % 2] += p[turn % 2];

        if score[turn % 2] >= 1000 {
            return i * 3 * score[(turn + 1) % 2];
        }
        turn += 1
    }
    panic!()
}

#[aocio]
pub fn large((p1, p2): Tuple<Tuple<_, ": ", usize>, "\n", Tuple<_, ": ", usize>>) -> i64 {
    let mut dp = vec![vec![vec![vec![vec![vec![0; 3]; 11]; 22]; 11]; 22]; 2];
    dp[0][0][p1][0][p2][0] = 1i64;

    let mut win1 = 0;
    let mut win2 = 0;
    for s1 in 0..22 {
        for s2 in 0..22 {
            for roll in 0..3 {
                for p1 in 0..11 {
                    for p2 in 0..11 {
                        for turn in 0..2 {
                            let v = dp[turn][s1][p1][s2][p2][roll];
                            if v == 0 {
                                continue;
                            }
                            if roll == 0 && (s1 >= 21 || s2 >= 21) {
                                if s1 >= 21 {
                                    win1 += v;
                                } else {
                                    win2 += v;
                                }
                                continue;
                            }
                            // println!("{} {} {} {} {} {}", &turn, &s1, &p1, &s2, &p2, &roll);
                            for d in 1..=3 {
                                let mut nturn = turn;
                                let mut ns1 = s1;
                                let mut np1 = p1;
                                let mut ns2 = s2;
                                let mut np2 = p2;
                                let mut nroll = roll + 1;

                                if turn == 0 {
                                    np1 = (np1 + d) % 10;
                                    if np1 == 0 {
                                        np1 += 10;
                                    }
                                } else {
                                    np2 = (np2 + d) % 10;
                                    if np2 == 0 {
                                        np2 += 10;
                                    }
                                }

                                if nroll == 3 {
                                    nroll = 0;
                                    nturn = (nturn + 1) % 2;

                                    if turn == 0 {
                                        ns1 += np1;
                                        ns1 = 21.min(ns1);
                                    } else {
                                        ns2 += np2;
                                        ns2 = 21.min(ns2);
                                    }
                                }
                                dp[nturn][ns1][np1][ns2][np2][nroll] += v;
                            }
                        }
                    }
                }
            }
        }
    }
    win1.max(win2)
}

aoc_test!(
    2021,
    21,
    "Player 1 starting position: 4
Player 2 starting position: 8",
    739785,
    444356092776315i64
);
