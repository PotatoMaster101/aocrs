use std::fmt::{Display, Formatter};
use std::ops::{Add, Mul, Neg, Sub};
use num::traits::{WrappingAdd, WrappingMul, WrappingNeg, WrappingSub};

/// Represents a position in a 2D space.
#[derive(Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Pos<T> {
    pub x: T,
    pub y: T,
}

impl<T: Default> Default for Pos<T> {
    #[inline]
    fn default() -> Self {
        Self { x: T::default(), y: T::default() }
    }
}

impl<T: Display> Display for Pos<T> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<T: WrappingAdd<Output = T>> Add for Pos<T> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self { x: self.x.wrapping_add(&rhs.x), y: self.y.wrapping_add(&rhs.y) }
    }
}

impl<T: WrappingSub<Output = T>> Sub for Pos<T> {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self { x: self.x.wrapping_sub(&rhs.x), y: self.y.wrapping_sub(&rhs.y) }
    }
}

impl<T: WrappingMul<Output = T>> Mul<T> for Pos<T> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        Self { x: self.x.wrapping_mul(&rhs), y: self.y.wrapping_mul(&rhs) }
    }
}

impl<T: WrappingNeg> Neg for Pos<T> {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self { x: self.x.wrapping_neg(), y: self.y.wrapping_neg() }
    }
}

impl From<u8> for Pos<i8> {
    #[inline]
    fn from(value: u8) -> Self {
        match value {
            b'<' => Self { x: -1, y: 0 },
            b'>' => Self { x: 1, y: 0 },
            b'v' | b'V' => Self { x: 0, y: 1 },
            _ => Self { x: 0, y: -1 },
        }
    }
}

impl<T: Copy + WrappingAdd + WrappingSub> Pos<T> {
    /// Returns the neighbours that forms a cross.
    #[inline]
    pub fn crosses(&self, dist: T) -> [Self; 4] {
        [
            Self { x: self.x, y: self.y.wrapping_add(&dist) },
            Self { x: self.x, y: self.y.wrapping_sub(&dist) },
            Self { x: self.x.wrapping_add(&dist), y: self.y },
            Self { x: self.x.wrapping_sub(&dist), y: self.y },
        ]
    }

    /// Returns the neighbours that forms an X.
    #[inline]
    pub fn diagonals(&self, dist: T) -> [Self; 4] {
        [
            Self { x: self.x.wrapping_add(&dist), y: self.y.wrapping_add(&dist) },
            Self { x: self.x.wrapping_sub(&dist), y: self.y.wrapping_add(&dist) },
            Self { x: self.x.wrapping_add(&dist), y: self.y.wrapping_sub(&dist) },
            Self { x: self.x.wrapping_sub(&dist), y: self.y.wrapping_sub(&dist) },
        ]
    }
}

impl<T: Copy + WrappingNeg> Pos<T> {
    /// Returns the current [`Pos<T>`] turned 90 degrees clockwise.
    #[inline]
    pub fn clockwise(&self) -> Self {
        Self { x: self.y, y: self.x.wrapping_neg() }
    }

    /// Returns the current [`Pos<T>`] turned 90 degrees counterclockwise.
    #[inline]
    pub fn counterclockwise(&self) -> Self {
        Self { x: self.y.wrapping_neg(), y: self.x }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_default() {
        assert_eq!(Pos::default(), Pos { x: 0, y: 0 });
    }

    #[test]
    fn test_display() {
        let sut = Pos { x: 1, y: -2 };
        assert_eq!(format!("{}", sut), "(1, -2)");
    }

    #[test]
    fn test_add() {
        let sut = Pos { x: 1, y: 2 } + Pos { x: 3, y: 4 };
        assert_eq!(sut, Pos { x: 4, y: 6 });

        let sut = Pos { x: -1, y: -2 } + Pos { x: -3, y: -4 };
        assert_eq!(sut, Pos { x: -4, y: -6 });

        let sut = Pos { x: 1, y: 2 } + Pos { x: i32::MAX, y: i32::MAX };
        assert_eq!(sut, Pos { x: i32::MIN, y: i32::MIN + 1 });
    }

    #[test]
    fn test_sub() {
        let sut = Pos { x: 1, y: 2 } - Pos { x: 3, y: 4 };
        assert_eq!(sut, Pos { x: -2, y: -2 });

        let sut = Pos { x: -1, y: -2 } - Pos { x: -3, y: -4 };
        assert_eq!(sut, Pos { x: 2, y: 2 });

        let sut = Pos { x: -1, y: -2 } - Pos { x: i32::MAX, y: i32::MAX };
        assert_eq!(sut, Pos { x: i32::MIN, y: i32::MAX });
    }

    #[test]
    fn test_mul() {
        let sut = Pos { x: 1, y: 2 } * -3;
        assert_eq!(sut, Pos { x: -3, y: -6 });

        let sut = Pos { x: -1, y: -2 } * 4;
        assert_eq!(sut, Pos { x: -4, y: -8 });

        let sut = Pos { x: 1, y: 2 } * i32::MAX;
        assert_eq!(sut, Pos { x: i32::MAX, y: -2 });
    }

    #[test]
    fn test_neg() {
        let sut = -Pos { x: 1, y: 2 };
        assert_eq!(sut, Pos { x: -1, y: -2 });

        let sut = -Pos { x: 1u32, y: 2u32 };
        assert_eq!(sut, Pos { x: u32::MAX, y: u32::MAX - 1 });
    }

    #[test]
    fn test_from_u8() {
        assert_eq!(Pos::from(b'<'), Pos { x: -1, y: 0 });
        assert_eq!(Pos::from(b'>'), Pos { x: 1, y: 0 });
        assert_eq!(Pos::from(b'v'), Pos { x: 0, y: 1 });
        assert_eq!(Pos::from(b'V'), Pos { x: 0, y: 1 });
        assert_eq!(Pos::from(b'^'), Pos { x: 0, y: -1 });
        assert_eq!(Pos::from(b'x'), Pos { x: 0, y: -1 });
    }

    #[test]
    fn test_crosses() {
        let sut = Pos { x: 0u32, y: 0u32 }.crosses(1);
        assert!(sut.contains(&Pos { x: 1, y: 0 }));
        assert!(sut.contains(&Pos { x: 0, y: 1 }));
        assert!(sut.contains(&Pos { x: u32::MAX, y: 0 }));
        assert!(sut.contains(&Pos { x: 0, y: u32::MAX }));

        let sut = Pos { x: 0u32, y: 0u32 }.crosses(3);
        assert!(sut.contains(&Pos { x: 3, y: 0 }));
        assert!(sut.contains(&Pos { x: 0, y: 3 }));
        assert!(sut.contains(&Pos { x: u32::MAX - 2, y: 0 }));
        assert!(sut.contains(&Pos { x: 0, y: u32::MAX - 2 }));
    }

    #[test]
    fn test_diagonals() {
        let sut = Pos { x: 0u32, y: 0u32 }.diagonals(1);
        assert!(sut.contains(&Pos { x: 1, y: 1 }));
        assert!(sut.contains(&Pos { x: u32::MAX, y: 1 }));
        assert!(sut.contains(&Pos { x: 1, y: u32::MAX }));
        assert!(sut.contains(&Pos { x: u32::MAX, y: u32::MAX }));

        let sut = Pos { x: 0u32, y: 0u32 }.diagonals(3);
        assert!(sut.contains(&Pos { x: 3, y: 3 }));
        assert!(sut.contains(&Pos { x: u32::MAX - 2, y: 3 }));
        assert!(sut.contains(&Pos { x: 3, y: u32::MAX - 2 }));
        assert!(sut.contains(&Pos { x: u32::MAX - 2, y: u32::MAX - 2 }));
    }

    #[test]
    fn test_clockwise() {
        let sut = Pos { x: 1, y: 0 }.clockwise();
        assert_eq!(sut, Pos { x: 0, y: -1 });

        let sut = Pos { x: 1, y: 1 }.clockwise();
        assert_eq!(sut, Pos { x: 1, y: -1 });
    }

    #[test]
    fn test_counterclockwise() {
        let sut = Pos { x: 1, y: 0 }.counterclockwise();
        assert_eq!(sut, Pos { x: 0, y: 1 });

        let sut = Pos { x: -1, y: 1 }.counterclockwise();
        assert_eq!(sut, Pos { x: -1, y: -1 });
    }
}
