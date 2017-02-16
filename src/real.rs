use std::fmt;
use std::ops::{Add, Sub, Mul, Div};
// use core::ops;

/// The ammount of bits which we shift the 
const FRACTION_BITS: i32 = 1 << 16; // 65536
const SHIFT: i32 = 16;
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
    /// `"3.14"`
    ///
    /// `"3."`
    ///
    /// `"3"`
    pub fn parse(input: &str) -> Result<Self, ()> {
        let dot = input.find('.').unwrap_or_else(|| input.len());
        let (msb, lsb) = input.split_at(dot);
        let frac = (lsb.parse::<f32>().unwrap_or(0.0) * FRACTION_BITS as f32) as i32;
        let int = msb.parse::<i32>().or(Err(()))?;
        if int > FRACTION_BITS {
            return Err(()); // too large
        }
        let real = if int >= 0 {
            (int << SHIFT) + frac
        } else {
            (int << SHIFT) - frac
        };
        Ok(Real { value: real })
    }

    /// Converts a `i32` to a Real
    pub fn from_int(i: i32) -> Self {
        Real { value: i << SHIFT }
    }

    /// Converts a `f32` to a Real
    pub fn from_float(f: f32) -> Self {
        Real { value: (f * FRACTION_BITS as f32) as i32 }
    }
}

// Arithmetic
// Source: http://x86asm.net/articles/fixed-point-arithmetic-and-tricks/

impl Add for Real {
    type Output = Real;
    fn add(self, rhs: Real) -> Self::Output {
        Real {
            value: self.value + rhs.value
        }
    } 
}

impl Sub for Real {
    type Output = Real;
    fn sub(self, rhs: Real) -> Self::Output {
        Real {
            value: self.value - rhs.value
        }
    } 
}

impl Mul for Real {
    type Output = Real;
    fn mul(self, rhs: Real) -> Self::Output {
        let a = self.value as i64;
        let b = rhs.value as i64;
        Real {
            value: (a * b >> SHIFT) as i32
        }
    } 
}

impl Div for Real {
    type Output = Real;
    fn div(self, rhs: Real) -> Self::Output {
        let a = (self.value as i64) << SHIFT;
        let b = rhs.value as i64;
        Real {
            value: (a / b) as i32
        }
    } 
}

impl fmt::Display for Real {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value as f32 / FRACTION_BITS as f32)
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