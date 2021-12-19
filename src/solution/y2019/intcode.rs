use std::collections::VecDeque;

pub struct Program {
    pub mem: Vec<i64>,
    ip: usize,
    input: VecDeque<i64>,
    output: VecDeque<i64>,
    base: i64,
}

impl Program {
    pub fn new(mut mem: Vec<i64>) -> Program {
        mem.resize(10000, 0);
        Program {
            mem,
            ip: 0,
            input: VecDeque::new(),
            output: VecDeque::new(),
            base: 0,
        }
    }
    fn next(&mut self) -> i64 {
        let res = self.mem[self.ip];
        self.ip += 1;
        res
    }
    fn instr(&mut self) -> Instr {
        let x = self.next();
        let op = (x % 100) as usize;
        let first = x / 100 % 10;
        let second = x / 1000 % 10;
        let third = x / 10000 % 10;
        Instr {
            op,
            mode: (first as u8, second as u8, third as u8),
        }
    }
    fn store(&mut self, v: i64, mode: u8) {
        let x = self.next();
        let i = match mode {
            0 => x,
            2 => self.base + x,
            _ => panic!(),
        };
        self.mem[i as usize] = v;
    }
    fn value(&mut self, mode: u8) -> i64 {
        let i = self.next();
        match mode {
            0 => self.mem[i as usize],
            1 => i,
            2 => self.mem[(self.base + i) as usize],
            _ => panic!(),
        }
    }
    pub fn write(&mut self, v: i64) {
        self.input.push_back(v)
    }
    pub fn read(&mut self) -> Option<i64> {
        self.run();
        self.output.pop_front()
    }
    pub fn read_all(&mut self) -> Vec<i64> {
        let mut res = vec![];
        while let Some(x) = self.read() {
            res.push(x);
        }
        res
    }
    fn input(&mut self) -> Option<i64> {
        self.input.pop_front()
    }
    fn output(&mut self, v: i64) {
        self.output.push_back(v)
    }
    fn step(&mut self) -> State {
        let i = self.instr();
        match i.op {
            1 => {
                let v = self.value(i.mode.0) + self.value(i.mode.1);
                self.store(v, i.mode.2);
            }
            2 => {
                let v = self.value(i.mode.0) * self.value(i.mode.1);
                self.store(v, i.mode.2);
            }
            3 => {
                let v = self.input();
                if let Some(x) = v {
                    self.store(x, i.mode.0);
                } else {
                    self.ip -= 1;
                    return State::Wait;
                }
            }
            4 => {
                let v = self.value(i.mode.0);
                self.output(v);
            }
            5 => {
                let v = self.value(i.mode.0);
                let p = self.value(i.mode.1);
                if v != 0 {
                    self.ip = p as usize;
                }
            }
            6 => {
                let v = self.value(i.mode.0);
                let p = self.value(i.mode.1);
                if v == 0 {
                    self.ip = p as usize;
                }
            }
            7 => {
                let v = self.value(i.mode.0);
                let u = self.value(i.mode.1);
                self.store(if v < u { 1 } else { 0 }, i.mode.2);
            }
            8 => {
                let v = self.value(i.mode.0);
                let u = self.value(i.mode.1);
                self.store(if v == u { 1 } else { 0 }, i.mode.2);
            }
            9 => {
                self.base += self.value(i.mode.0);
            }
            99 => {
                self.ip -= 1;
                return State::Halt;
            }
            _ => panic!(),
        }
        State::Ok
    }
    fn run(&mut self) -> State {
        loop {
            match self.step() {
                State::Ok => (),
                x => return x,
            }
        }
    }
    pub fn run_until_halt(&mut self) -> i64 {
        while self.step() != State::Halt {}
        self.mem[0]
    }
    pub fn running(&mut self) -> bool {
        self.run() != State::Halt || !self.output.is_empty()
    }
}

struct Instr {
    op: usize,
    // first, second, third
    mode: (u8, u8, u8),
}

#[derive(PartialEq, Eq)]
pub enum State {
    Ok,
    Halt,
    Wait,
}
