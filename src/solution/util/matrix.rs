pub type Matrix<S> = Vec<Vec<S>>;

pub trait RingElem: num::One + num::Zero + num::CheckedAdd + num::CheckedMul + Copy {}
impl<T: num::One + num::Zero + num::CheckedAdd + num::CheckedMul + Copy> RingElem for T {}

pub fn new<S: RingElem>(n: usize, m: usize) -> Matrix<S> {
    vec![vec![S::zero(); m]; n]
}

pub fn identity<S: RingElem>(n: usize) -> Matrix<S> {
    let mut m = new(n, n);
    for i in 0..n {
        m[i][i] = S::one();
    }
    m
}

pub fn mul<S: RingElem>(m1: &Matrix<S>, m2: &Matrix<S>) -> Matrix<S> {
    let mut res = new(m1.len(), m2[0].len());
    for i in 0..m1.len() {
        for j in 0..m2[0].len() {
            for k in 0..m2.len() {
                res[i][j] = res[i][j] + m1[i][k] * m2[k][j];
            }
        }
    }
    res
}

pub fn pow<S: RingElem>(m: &Matrix<S>, k: usize) -> Matrix<S> {
    let n = m.len();
    if k == 0 {
        identity(n)
    } else if k % 2 == 1 {
        mul(&pow(m, k - 1), m)
    } else {
        pow(&mul(m, m), k / 2)
    }
}

pub fn mul_vec<S: RingElem>(m: &Matrix<S>, v: &Vec<S>) -> Vec<S> {
    let mut u = vec![S::zero(); m.len()];
    for i in 0..m.len() {
        for j in 0..v.len() {
            u[i] = u[i] + (m[i][j] * v[j]);
        }
    }
    u
}
