use std::str::FromStr;

pub enum Instr {
    Down(i32),
    Forward(i32),
    Up(i32),
}

impl FromStr for Instr {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ss = s.split(" ");
        let cmd = ss.next().unwrap();
        let n: i32 = ss.next().unwrap().parse()?;
        Ok(match cmd {
            "down" => Self::Down(n),
            "forward" => Self::Forward(n),
            "up" => Self::Up(n),
            _ => anyhow::bail!("{} not known", cmd),
        })
    }
}

pub fn small(a: Vec<Instr>) -> i32 {
    let (mut x, mut y) = (0i32, 0i32);
    for i in a {
        match i {
            Instr::Down(k) => x += k,
            Instr::Forward(k) => y += k,
            Instr::Up(k) => x -= k,
        }
    }
    return x * y;
}

pub fn large(a: Vec<Instr>) -> i32 {
    let mut aim = 0;
    let mut depth = 0;
    let mut pos = 0;
    for i in a {
        match i {
            Instr::Down(k) => aim += k,
            Instr::Up(k) => aim -= k,
            Instr::Forward(k) => {
                pos += k;
                depth += k * aim
            }
        }
    }
    return pos * depth;
}
