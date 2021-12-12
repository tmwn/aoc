pub fn small(a: Vec<usize>) -> usize {
    let mut b = vec![false; 2021];
    for x in a {
        b[x] = true;
    }
    for i in 0..2020 {
        if b[i] && b[2020 - i] {
            return i * (2020 - i);
        }
    }
    0
}

pub fn large(a: Vec<usize>) -> usize {
    let mut b = vec![false; 2021];
    for x in a {
        b[x] = true;
    }
    for i in 0..2020 {
        for j in 0..(2020 - i) {
            if b[i] && b[j] && b[2020 - i - j] {
                return i * j * (2020 - i - j);
            }
        }
    }
    0
}
