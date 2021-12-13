use std::str::FromStr;

#[derive(Debug, Hash, PartialEq, PartialOrd, Eq, Ord, Clone)]
pub struct Point<I> {
    pub x: I,
    pub y: I,
}

impl<I: FromStr> FromStr for Point<I> {
    type Err = I::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ss = s.split(',').map(|x| x.parse());
        let x = ss.next().unwrap()?;
        let y = ss.next().unwrap()?;
        Ok(Self { x, y })
    }
}
