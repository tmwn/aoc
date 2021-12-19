use aocio::aocio;

use crate::solution::aoc_test;

#[aocio]
pub fn small(a: Vec<String>) -> i32 {
    let nums = a.iter().map(|s| parse(s));
    let result = nums.reduce(add).unwrap();
    magnitude(result)
}

#[aocio]
pub fn large(a: Vec<String>) -> i32 {
    let nums: Vec<_> = a.iter().map(|s| parse(s)).collect();
    let n = nums.len();
    let mut res = 0;
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            res = res.max(magnitude(add(nums[i].clone(), nums[j].clone())));
        }
    }
    res
}

type Num = Vec<(i32, usize)>;

fn parse(s: &str) -> Num {
    let mut d = 0;
    let mut res: Num = vec![];
    for b in s.bytes() {
        match b {
            b'[' => d += 1,
            b']' => d -= 1,
            b',' => (),
            x => res.push(((x - b'0') as i32, d)),
        }
    }
    res
}

fn add(a: Num, b: Num) -> Num {
    let mut c = [a, b].concat();
    c.iter_mut().for_each(|x| x.1 += 1);
    reduce(c)
}

fn reduce(mut a: Num) -> Num {
    let n = a.len();
    // Expand
    for i in 0..n {
        if a[i].1 == 5 {
            if i >= 1 {
                a[i - 1].0 += a[i].0;
            }
            if i + 2 < n {
                a[i + 2].0 += a[i + 1].0;
            }
            a[i] = (0, 4);
            a.remove(i + 1);
            return reduce(a);
        }
    }
    // Split
    for i in 0..n {
        if a[i].0 >= 10 {
            let (v, d) = a[i];
            a[i] = (v / 2, d + 1);
            a.insert(i + 1, (v - v / 2, d + 1));
            return reduce(a);
        }
    }
    a
}

fn magnitude(a: Num) -> i32 {
    let mut stack: Num = vec![];
    for x in a {
        stack.push(x);
        while stack.len() >= 2 {
            let n = stack.len();
            if stack[n - 2].1 == stack[n - 1].1 {
                stack[n - 2].1 -= 1;
                stack[n - 2].0 = stack[n - 2].0 * 3 + stack[n - 1].0 * 2;
                stack.pop().unwrap();
            } else {
                break;
            }
        }
    }
    stack[0].0
}

aoc_test!(
    2021,
    18,
    "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
    [[[5,[2,8]],4],[5,[[9,9],0]]]
    [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
    [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
    [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
    [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
    [[[[5,4],[7,7]],8],[[8,3],8]]
    [[9,3],[[9,9],[6,[4,9]]]]
    [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
    [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]",
    4140,
    3993
);
