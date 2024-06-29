// TODO: implement the necessary traits to make the test compile and pass.
//  You *can't* modify the test.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WrappingU32 {
    value: u32,
}

impl std::ops::Add for WrappingU32 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        // wrapping_add handles integer overflow
        // in a safe and predictable manner, but it can't be used
        // in all aplications (finance, for instance)
        // The "+" operator implements wrapping add
        Self::new(self.value.wrapping_add(rhs.value))
    }
}

impl WrappingU32 {
    pub fn new(value: u32) -> Self {
        Self { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ops() {
        let x = WrappingU32::new(42);
        let y = WrappingU32::new(31);
        let z = WrappingU32::new(u32::MAX);
        assert_eq!(x + y + y + z, WrappingU32::new(103));
    }
}
