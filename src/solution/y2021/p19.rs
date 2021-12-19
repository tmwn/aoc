use aocio::aocio;
use itertools::Itertools;

use crate::solution::{aoc_test, util::matrix::Matrix};

pub fn small(a: String) -> usize {
    solve(a.parse().unwrap()).0
}
pub fn large(a: String) -> i32 {
    solve(a.parse().unwrap()).1
}

#[aocio]
pub fn solve(
    bs: Vec<Tuple<_, " scanner ", _, " ---", Vec<Tuple<i32, ",", i32, ",", i32>>>, "\n\n">,
) -> (usize, i32) {
    let oris = all_orientations();
    let solver = Solver { oris, bs };

    let mut scs: Vec<Option<Scanner>> = vec![None; solver.bs.len()];
    scs[0] = Some(Scanner::new((0, 0, 0), &solver.bs[0]));

    solver.dfs(&mut scs, 0);

    let scs: Vec<_> = scs.into_iter().map(|x| x.unwrap()).collect();

    let mut all = vec![];
    for sc in scs.iter() {
        for b in &sc.bs {
            let p = add(sc.pos, *b);
            all.push(p);
        }
    }
    all.sort_unstable();
    all.dedup();
    let small = all.len();

    let mut large = 0;
    for sc1 in scs.iter() {
        for sc2 in scs.iter() {
            let x = sub(sc1.pos, sc2.pos);
            large = large.max(x.0.abs() + x.1.abs() + x.2.abs());
        }
    }
    (small, large)
}

struct Solver {
    oris: Vec<Ori>,
    bs: Vec<Vec<P>>,
}

impl Solver {
    fn dfs(&self, scs: &mut Vec<Option<Scanner>>, i: usize) {
        for j in 0..scs.len() {
            if scs[j].is_some() {
                continue;
            }
            let sc = scs[i].as_ref().unwrap();
            if let Some((rotated, d)) = self.overlap(&sc.bs, j) {
                let pos = add(sc.pos, d);
                let sc = Scanner::new(pos, &rotated);
                scs[j] = Some(sc);
                self.dfs(scs, j);
            }
        }
    }
    fn overlap(&self, sorted_base: &Vec<P>, examine: usize) -> Option<(Vec<P>, P)> {
        for o in self.oris.iter() {
            let rotated: Vec<_> = self.bs[examine].iter().map(|x| mul(*x, o)).collect();

            for p1 in sorted_base.iter() {
                for p2 in rotated.iter() {
                    let d = sub(*p1, *p2);

                    let mut count = 0;
                    for x in rotated.iter() {
                        if sorted_base.binary_search(&add(*x, d)).is_ok() {
                            count += 1;
                        }
                    }
                    if count >= 12 {
                        return Some((rotated, d));
                    }
                }
            }
        }
        None
    }
}

fn all_orientations() -> Vec<Ori> {
    let mut oris: Vec<Ori> = vec![];
    for perm in [0, 1, 2].iter().permutations(3) {
        for i in [-1, 1] {
            for j in [-1, 1] {
                for k in [-1, 1] {
                    let mut m = vec![vec![0; 3]; 3];
                    for d in 0..3 {
                        m[d][*perm[d]] = [i, j, k][d];
                    }
                    let mut sign = i * j * k;
                    for d in 0..3 {
                        for e in 0..d {
                            if perm[e] > perm[d] {
                                sign *= -1;
                            }
                        }
                    }
                    if sign == 1 {
                        oris.push(m);
                    }
                }
            }
        }
    }
    oris
}

type P = (i32, i32, i32);
type Ori = Matrix<i32>;

fn add(a: P, b: P) -> P {
    (a.0 + b.0, a.1 + b.1, a.2 + b.2)
}
fn sub(a: P, b: P) -> P {
    (a.0 - b.0, a.1 - b.1, a.2 - b.2)
}
fn mul(a: P, b: &Ori) -> P {
    let v = &[a.0, a.1, a.2];
    let mut res = [0; 3];
    for i in 0..3 {
        for j in 0..3 {
            res[i] += b[i][j] * v[j];
        }
    }
    (res[0], res[1], res[2])
}

#[derive(Debug, Clone)]
struct Scanner {
    pos: P,
    bs: Vec<P>, // sorted
}

impl Scanner {
    fn new(pos: P, bs: &[P]) -> Scanner {
        let mut bs = bs.to_owned();
        bs.sort_unstable();
        Scanner { pos, bs }
    }
}

aoc_test!(
    2021,
    19,
    "--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14",
    79,
    3621
);
