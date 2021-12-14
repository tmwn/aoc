pub type Matrix<S> = Vec<Vec<S>>;
type MatrixRef<S> = [Vec<S>];

pub trait RingElem: num::One + num::Zero + num::CheckedAdd + num::CheckedMul + Copy {}
impl<T: num::One + num::Zero + num::CheckedAdd + num::CheckedMul + Copy> RingElem for T {}

pub fn new<S: RingElem>(n: usize, m: usize) -> Matrix<S> {
    vec![vec![S::zero(); m]; n]
}

pub fn identity<S: RingElem>(n: usize) -> Matrix<S> {
    let mut m = new(n, n);
    for (i, r) in m.iter_mut().enumerate() {
        r[i] = S::one();
    }
    m
}

pub fn mul<S: RingElem>(m1: &MatrixRef<S>, m2: &MatrixRef<S>) -> Matrix<S> {
    let mut res = new(m1.len(), m2[0].len());
    for (i, r1) in m1.iter().enumerate() {
        for j in 0..m2[0].len() {
            for (k, r2) in m2.iter().enumerate() {
                res[i][j] = res[i][j] + r1[k] * r2[j];
            }
        }
    }
    res
}

pub fn pow<S: RingElem>(m: &MatrixRef<S>, k: usize) -> Matrix<S> {
    let n = m.len();
    if k == 0 {
        identity(n)
    } else if k % 2 == 1 {
        mul(&pow(m, k - 1), m)
    } else {
        pow(&mul(m, m), k / 2)
    }
}

pub fn mul_vec<S: RingElem>(m: &MatrixRef<S>, v: &[S]) -> Vec<S> {
    let mut u = vec![S::zero(); m.len()];
    for (i, mi) in m.iter().enumerate() {
        for (j, x) in v.iter().enumerate() {
            u[i] = u[i] + (mi[j] * *x);
        }
    }
    u
}
