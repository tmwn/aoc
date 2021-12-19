use aocio::aocio;
use itertools::Itertools;

use crate::solution::{
    aoc_test,
    util::matrix::{self, Matrix},
};

#[aocio]
pub fn small(
    a: Vec<Tuple<String, " scanner ", usize, " ---", Vec<Tuple<i32, ",", i32, ",", i32>>>, "\n\n">,
) -> usize {
    let n = a.len();
    let mut scs: Vec<Option<Scanner>> = vec![None; n];

    scs[0] = Some(Scanner {
        pos: (0, 0, 0),
        orientation: matrix::identity(3),
    });

    let mut oris: Vec<Matrix<i32>> = vec![];
    for perm in [0, 1, 2].iter().permutations(3) {
        for i in [-1, 1] {
            for j in [-1, 1] {
                for k in [-1, 1] {
                    let mut m = vec![vec![0; 3]; 3];
                    for d in 0..3 {
                        m[d][*perm[d]] = [i, j, k][d];
                    }
                    oris.push(m);
                }
            }
        }
    }

    dfs(&mut scs, 0, &a, &oris);

    let mut all = vec![];
    for i in 0..n {
        let sc = scs[i].clone().unwrap();
        for b in &a[i].2 {
            let p = add(sc.pos, mul(*b, &sc.orientation));
            all.push(p);
        }
    }
    all.sort();
    all.dedup();
    let small = all.len();

    let mut res = 0;
    for i in 0..n {
        for j in 0..n {
            let x = sub(scs[i].as_ref().unwrap().pos, scs[j].as_ref().unwrap().pos);
            res = res.max(x.0.abs() + x.1.abs() + x.2.abs());
        }
    }
    res as usize
}

fn dfs(
    scs: &mut Vec<Option<Scanner>>,
    i: usize,
    a: &Vec<(String, usize, Vec<P>)>,
    oris: &Vec<Ori>,
) {
    let sc = scs[i].clone().unwrap();
    let b1 = &a[i].2;
    for j in 0..scs.len() {
        if scs[j].is_some() {
            continue;
        }
        if let Some((o, d)) = overlap(&b1, &a[j].2, oris) {
            scs[j] = Some(Scanner {
                orientation: mul_ori(&sc.orientation, &o),
                pos: add(sc.pos, mul(d, &sc.orientation)),
            });
            dfs(scs, j, a, oris);
        }
    }
}

type P = (i32, i32, i32);
type Ori = Matrix<i32>;

fn overlap(a: &Vec<P>, b: &Vec<P>, oris: &Vec<Ori>) -> Option<(Ori, P)> {
    for o in oris {
        for i in 0..a.len() {
            for j in 0..b.len() {
                let d = sub(a[i], mul(b[j], o));
                let m = update(&b, o, d);
                let mut count = 0;
                for x in a {
                    if m.binary_search(x).is_ok() {
                        count += 1;
                    }
                }
                if count >= 12 {
                    return Some((o.clone(), d));
                }
            }
        }
    }
    None
}

fn update(a: &Vec<P>, o: &Ori, d: P) -> Vec<P> {
    let mut res: Vec<_> = a.into_iter().map(|x| add(mul(*x, o), d)).collect();
    res.sort();
    res
}

fn add(a: P, b: P) -> P {
    (a.0 + b.0, a.1 + b.1, a.2 + b.2)
}
fn sub(a: P, b: P) -> P {
    (a.0 - b.0, a.1 - b.1, a.2 - b.2)
}
fn mul(a: P, b: &Ori) -> P {
    let mut v = vec![a.0, a.1, a.2];
    let res = matrix::mul_vec(b, &v);
    (res[0], res[1], res[2])
}
fn mul_ori(a: &Ori, b: &Ori) -> Ori {
    matrix::mul(a, b)
}

#[derive(Clone, Debug)]
struct Scanner {
    orientation: Ori,
    pos: P,
}

#[aocio]
pub fn large(_: Vec<String>) -> i32 {
    todo!()
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
