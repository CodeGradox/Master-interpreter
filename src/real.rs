use std::fmt;
use std::ops::{Add, Sub, Mul, Div};

use error;
use error::Error::*;

/// This value is used when we want to convert a float to a real.
/// The float is multiplied with this value.
const FRACTION_VALUE: i32 = 1 << 16;
/// Maximus size of the int part.
/// Mask = 0111 1111 1111 1111
const MAX_SIZE: i32 = 0x7FFF;
const MIN_SIZE: i32 = 0xFFFF;
/// The ammount of bits which the int part must be shifted.
const SHIFT: i32 = 16;
/// Maskign for the fraction part. It is used to remove the int part.
const FRACTION_MASK: i32 = 0xFFFF;

/// A real with the format 16.16, meaning 16 bits for the integer
/// and 16 bits for the fraction. The int is signed, meaning
/// its range is `[-32768, 32767]`.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Real {
    value: i32,
}

impl Real {
    /// Parses a string to a `Real`.
    /// # Legal input examples
    /// `"3.14"`, `"3."`, `"3"`, `"."`, `".14"`
    pub fn parse(input: &str) -> error::Result<Real> {
        let dot = input.find('.').unwrap_or_else(|| input.len());
        if dot == 0 {
            return Err(BadRealLiteral);
        }
        let (msb, lsb) = input.split_at(dot);
        let int = msb.parse::<i32>()?;
        if int > MAX_SIZE {
            return Err(LargeInt);
        }
        // figure out how to check that a value is negative
        let frac = (lsb.parse::<f32>().unwrap_or(0.0) * FRACTION_VALUE as f32) as i32;
        let real = if (int >> SHIFT) >= 0 {
            (int << SHIFT) + frac
        } else {
            (int << SHIFT) - frac
        };
        Ok(Real { value: real })
    }
}

// Arithmetic
// Source: http://x86asm.net/articles/fixed-point-arithmetic-and-tricks/

impl Add for Real {
    type Output = Real;
    fn add(self, rhs: Real) -> Self::Output {
        Real { value: self.value + rhs.value }
    }
}

impl Sub for Real {
    type Output = Real;
    fn sub(self, rhs: Real) -> Self::Output {
        Real { value: self.value - rhs.value }
    }
}

impl Mul for Real {
    type Output = Real;
    fn mul(self, rhs: Real) -> Self::Output {
        // The reason we need to cast to i64 is because multiplying
        // two 16.16 FP will result in one 32.32 FP. Thus, we cast them
        // to a larger format and the shift the result back down
        // to the 16.16 format.
        let a = self.value as i64;
        let b = rhs.value as i64;
        Real { value: ((a * b) >> SHIFT) as i32 }
    }
}

impl Div for Real {
    type Output = Real;
    fn div(self, rhs: Real) -> Self::Output {
        // Same logic as with multiplication, but we only need to
        // shift one of the values. 32.32 / 16.16 = 16.16.
        let a = (self.value as i64) << SHIFT;
        let b = rhs.value as i64;
        Real { value: (a / b) as i32 }
    }
}

// ***** i32 *****

impl Add<i32> for Real {
    type Output = Real;
    fn add(self, rhs: i32) -> Self::Output {
        Real { value: self.value + (rhs << SHIFT) }
    }
}

impl Sub<i32> for Real {
    type Output = Real;
    fn sub(self, rhs: i32) -> Self::Output {
        Real { value: self.value - (rhs << SHIFT) }
    }
}

impl Mul<i32> for Real {
    type Output = Real;
    fn mul(self, rhs: i32) -> Self::Output {
        Real { value: self.value * rhs }
    }
}

impl Div<i32> for Real {
    type Output = Real;
    fn div(self, rhs: i32) -> Self::Output {
        Real { value: self.value / rhs }
    }
}

impl From<i32> for Real {
    fn from(n: i32) -> Self {
        Real { value: n << SHIFT }
    }
}

// ***** f32 *****

impl Add<f32> for Real {
    type Output = Real;
    fn add(self, rhs: f32) -> Self::Output {
        self + Real::from(rhs)
    }
}

impl Sub<f32> for Real {
    type Output = Real;
    fn sub(self, rhs: f32) -> Self::Output {
        self - Real::from(rhs)
    }
}

impl Mul<f32> for Real {
    type Output = Real;
    fn mul(self, rhs: f32) -> Self::Output {
        self * Real::from(rhs)
    }
}

impl Div<f32> for Real {
    type Output = Real;
    fn div(self, rhs: f32) -> Self::Output {
        self / Real::from(rhs)
    }
}

impl From<f32> for Real {
    fn from(f: f32) -> Self {
        Real { value: (f * FRACTION_VALUE as f32) as i32 }
    }
}

impl fmt::Display for Real {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value as f32 / FRACTION_VALUE as f32)
    }
}

impl fmt::Debug for Real {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "Real {{ value: msb:{} lsb:{} }}",
               self.value >> SHIFT,
               self.value & FRACTION_MASK)
    }
}
