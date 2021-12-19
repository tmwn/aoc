pub struct Program {
    mem: Vec<i32>,
    p: usize,
}

impl Program {
    pub fn new(mem: Vec<i32>) -> Program {
        Program { mem, p: 0 }
    }
    fn next(&mut self) -> i32 {
        let res = self.mem[self.p];
        self.p += 1;
        res
    }
    fn store(&mut self, v: i32) {
        let i = self.next() as usize;
        self.mem[i] = v;
    }
    fn value(&mut self) -> i32 {
        let i = self.next() as usize;
        self.mem[i]
    }
    pub fn run(&mut self) -> i32 {
        match self.next() {
            1 => {
                let v = self.value() + self.value();
                self.store(v);
            }
            2 => {
                let v = self.value() * self.value();
                self.store(v);
            }
            99 => return self.mem[0],
            _ => panic!(),
        }
        self.run()
    }
}
