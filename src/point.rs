use std::str::FromStr;


#[derive(Debug)]
pub struct Point<T> {
    pub x: T,
    pub y: T
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: FromStr> TryFrom<String> for Point<T>
    where <T as FromStr>::Err: core::fmt::Debug
{
    type Error = T::Err;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut values = value.split_whitespace();

        let x = values.next().unwrap().parse::<T>()?;
        let y = values.next().unwrap().parse::<T>()?;

        Ok(Self { x, y })
    }
}
