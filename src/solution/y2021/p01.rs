use aocio::aocio;

#[aocio]
pub fn small(a: Vec<i32>) -> i32 {
    let n = a.len();
    let mut res = 0;
    for i in 0..(n - 1) {
        if a[i] < a[i + 1] {
            res += 1;
        }
    }
    res
}

#[aocio]
pub fn large(a: Vec<i32>) -> i32 {
    let n = a.len();
    let mut res = 0;
    for i in 0..(n - 3) {
        if a[i] < a[i + 3] {
            res += 1;
        }
    }
    res
}
