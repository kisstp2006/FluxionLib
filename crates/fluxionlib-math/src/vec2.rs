//! Two-dimensional vector types and operations.

use std::ops::{Add, Div, Mul, Sub};

impl<T> Add for Vec2<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

/// A two-dimensional vector containing `x` and `y` components.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> {
    /// Creates a vector from its `x` and `y` components.
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

/// A two-dimensional vector containing 32-bit signed integers.
pub type Vec2i = Vec2<i32>;

/// A two-dimensional vector containing 32-bit floating-point values.
pub type Vec2f = Vec2<f32>;

/// A two-dimensional vector containing 64-bit floating-point values.
pub type Vec2d = Vec2<f64>;

impl<T> Sub for Vec2<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T> Mul for Vec2<T>
where
    T: Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl<T> Div for Vec2<T>
where
    T: Div<Output = T>,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.x / rhs.x, self.y / rhs.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_vectors_from_components() {
        let integer = Vec2i::new(1, 2);
        let float = Vec2f::new(1.5, 2.5);
        let double = Vec2d::new(3.5, 4.5);

        assert_eq!(integer, Vec2 { x: 1, y: 2 });
        assert_eq!(float, Vec2 { x: 1.5, y: 2.5 });
        assert_eq!(double, Vec2 { x: 3.5, y: 4.5 });
    }

    #[test]
    fn adds_vectors_component_wise() {
        let a = Vec2i::new(1, 2);
        let b = Vec2i::new(3, 4);

        assert_eq!(a + b, Vec2i::new(4, 6));
    }

    #[test]
    fn performs_component_wise_arithmetic() {
        let a = Vec2i::new(12, 8);
        let b = Vec2i::new(3, 2);

        assert_eq!(a - b, Vec2i::new(9, 6));
        assert_eq!(a * b, Vec2i::new(36, 16));
        assert_eq!(a / b, Vec2i::new(4, 4));
    }
}
