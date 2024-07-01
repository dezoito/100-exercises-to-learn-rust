// TODO: Define a new `SaturatingU16` type.
//   It should hold a `u16` value.
//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
//   It should support addition with a right-hand side of type
//   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`.
//   It should be possible to compare it with another `SaturatingU16` or a `u16`.
//   It should be possible to print its debug representation.
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SaturatingU16 {
    value: u16,
}

// Conversions
impl From<u8> for SaturatingU16 {
    fn from(value: u8) -> Self {
        SaturatingU16 {
            value: value.into(),
        }
    }
}

impl From<u16> for SaturatingU16 {
    fn from(value: u16) -> Self {
        SaturatingU16 { value }
    }
}

impl From<&u8> for SaturatingU16 {
    fn from(value: &u8) -> Self {
        SaturatingU16 {
            value: (*value).into(),
        }
    }
}

impl From<&u16> for SaturatingU16 {
    fn from(value: &u16) -> Self {
        SaturatingU16 {
            value: (*value).into(),
        }
    }
}

// Handle addition with Saturation
// we make one implementation for each type,
// without defining the trait (it already is in std)
// u8 is not necessary

// Casting add here just for clarity
impl std::ops::Add<SaturatingU16> for SaturatingU16 {
    type Output = Self;
    // no need to use saturating_add yet
    fn add(self, rhs: Self) -> Self::Output {
        self + rhs.value
    }
}

impl std::ops::Add<u16> for SaturatingU16 {
    type Output = SaturatingU16;

    // Prompt tells us to saturate for u16
    fn add(self, rhs: u16) -> Self::Output {
        Self {
            value: self.value.saturating_add(rhs),
        }
    }
}

impl std::ops::Add<&u16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: &u16) -> Self::Output {
        self + *rhs
    }
}

impl std::ops::Add<&SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: &SaturatingU16) -> Self::Output {
        self + *rhs
    }
}

// Implementing Equality comparison with u16
// we already derive in the Struct declaration
// so no need to implement self comparison
impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, other: &u16) -> bool {
        self.value == *other
    }
}

// for reference, here are the tests used
// let a: SaturatingU16 = (&10u8).into();
// let b: SaturatingU16 = 5u8.into();
// let c: SaturatingU16 = u16::MAX.into();
// let d: SaturatingU16 = (&1u16).into();

// assert_eq!(a + b, SaturatingU16::from(15u16));
// assert_eq!(a + c, SaturatingU16::from(u16::MAX));
// assert_eq!(a + d, SaturatingU16::from(11u16));
// assert_eq!(a + a, 20u16);
// assert_eq!(a + 5u16, 15u16);
// assert_eq!(a + &u16::MAX, SaturatingU16::from(u16::MAX));
