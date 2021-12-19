use std::collections::VecDeque;

pub struct Program {
    pub mem: Vec<i32>,
    ip: usize,
    input: VecDeque<i32>,
    output: VecDeque<i32>,
}

impl Program {
    pub fn new(mem: Vec<i32>) -> Program {
        Program {
            mem,
            ip: 0,
            input: VecDeque::new(),
            output: VecDeque::new(),
        }
    }
    fn next(&mut self) -> i32 {
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
    fn store(&mut self, v: i32) {
        let i = self.next() as usize;
        self.mem[i] = v;
    }
    fn value(&mut self, mode: u8) -> i32 {
        let i = self.next();
        match mode {
            0 => self.mem[i as usize],
            1 => i,
            _ => panic!(),
        }
    }
    pub fn write(&mut self, v: i32) {
        self.input.push_back(v)
    }
    pub fn read(&mut self) -> Option<i32> {
        self.output.pop_front()
    }
    fn input(&mut self) -> Option<i32> {
        self.input.pop_front()
    }
    fn output(&mut self, v: i32) {
        self.output.push_back(v)
    }
    pub fn step(&mut self) -> State {
        let i = self.instr();
        match i.op {
            1 => {
                let v = self.value(i.mode.0) + self.value(i.mode.1);
                self.store(v);
            }
            2 => {
                let v = self.value(i.mode.0) * self.value(i.mode.1);
                self.store(v);
            }
            3 => {
                let v = self.input();
                if let Some(x) = v {
                    self.store(x);
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
                self.store(if v < u { 1 } else { 0 });
            }
            8 => {
                let v = self.value(i.mode.0);
                let u = self.value(i.mode.1);
                self.store(if v == u { 1 } else { 0 });
            }
            99 => return State::Halt,
            _ => panic!(),
        }
        return State::Ok;
    }
    pub fn run(&mut self) -> State {
        loop {
            match self.step() {
                State::Ok => (),
                x => return x,
            }
        }
    }
    pub fn run_until_halt(&mut self) -> i32 {
        while self.step() != State::Halt {
            ()
        }
        self.mem[0]
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
