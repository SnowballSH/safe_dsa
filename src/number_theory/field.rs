/*!
Number Theory

Field

Defines:
- trait [Field]
- struct [Fp]

Implements [Field] for:
- [Fp]
- [f64]
*/

use contracts::debug_invariant;
use std::ops::{Add, Div, Mul, Neg, Sub};

pub struct Assert<const B: bool>;
pub trait IsTrue {}
impl IsTrue for Assert<true> {}

const fn is_prime_u64(p: u64) -> bool {
    if p < 2 {
        return false;
    }
    if p % 2 == 0 {
        return p == 2;
    }
    let mut d: u64 = 3;
    while d <= p / d {
        if p % d == 0 {
            return false;
        }
        d += 2;
    }
    true
}

// Ensure (P-1)^2 <= u64::MAX without overflowing
const fn square_bound_ok(p: u64) -> bool {
    if p == 0 {
        return false;
    }
    let m = p - 1;
    m * m <= u64::MAX
}

const fn valid_modulus(p: u64) -> bool {
    square_bound_ok(p) && is_prime_u64(p)
}

/// A math Field: supports +, -, *, / (except by 0), additive & multiplicative identities,
/// and inverses. Equality should be a proper equivalence relation.
pub trait Field:
    Copy
    + Clone
    + PartialEq
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Neg<Output = Self>
    + Sized
{
    /// Additive identity
    fn zero() -> Self;

    /// Multiplicative identity
    fn one() -> Self;

    /// Is element the additive identity?
    #[inline]
    fn is_zero(self) -> bool {
        self == Self::zero()
    }

    /// Multiplicative inverse (must panic or otherwise signal if `self` is zero).
    #[inline]
    fn inv(self) -> Self {
        Self::one() / self
    }

    /// Integer exponentiation
    #[inline]
    fn powi(self, n: i64) -> Self {
        if n == 0 {
            return Self::one();
        }
        if n < 0 {
            return self.inv().powi(-n);
        }
        let mut base = self;
        let mut e = n as u64;
        let mut acc = Self::one();
        while e > 0 {
            if e & 1 == 1 {
                acc = acc * base;
            }
            base = base * base;
            e >>= 1;
        }
        acc
    }
}

/// Integers modulo prime P
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Fp<const P: u64>(pub u64);

#[debug_invariant(valid_modulus(P))]
#[debug_invariant(self.0 < P)]
impl<const P: u64> Fp<P> {
    #[inline]
    pub fn new(x: u64) -> Self {
        Fp::<P>(x % P)
    }

    #[inline]
    pub fn value(self) -> u64 {
        self.0
    }

    #[inline]
    fn add_raw(self, rhs: Self) -> Self {
        let mut s = self.0 + rhs.0;
        if s >= P {
            s -= P;
        }
        Fp::<P>(s)
    }

    #[inline]
    fn sub_raw(self, rhs: Self) -> Self {
        if self.0 >= rhs.0 {
            Fp::<P>(self.0 - rhs.0)
        } else {
            Fp::<P>(self.0 + P - rhs.0)
        }
    }

    #[inline]
    fn mul_raw(self, rhs: Self) -> Self {
        // Safe in u64 because 0 <= a,b < P and (P-1)^2 <= u64::MAX
        Fp::<P>((self.0 * rhs.0) % P)
    }
}

impl<const P: u64> Add for Fp<P> {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        self.add_raw(rhs)
    }
}
impl<const P: u64> Sub for Fp<P> {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        self.sub_raw(rhs)
    }
}
impl<const P: u64> Mul for Fp<P> {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        self.mul_raw(rhs)
    }
}
impl<const P: u64> Div for Fp<P> {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        if rhs.0 == 0 {
            panic!("division by zero in Fp");
        }
        // Fermat's Little Theorem
        self * Fp::<P>::powi(rhs, (P - 2) as i64)
    }
}
impl<const P: u64> Neg for Fp<P> {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self::Output {
        if self.0 == 0 {
            self
        } else {
            Fp::<P>(P - self.0)
        }
    }
}

impl<const P: u64> Field for Fp<P> {
    #[inline]
    fn zero() -> Self {
        Fp::<P>(0)
    }
    #[inline]
    fn one() -> Self {
        Fp::<P>(1 % P)
    }
    #[inline]
    fn inv(self) -> Self {
        if self.0 == 0 {
            panic!("inverse of zero in Fp");
        }
        Fp::<P>::powi(self, (P - 2) as i64)
    }
}

impl Field for f64 {
    #[inline]
    fn zero() -> Self {
        0.0
    }
    #[inline]
    fn one() -> Self {
        1.0
    }

    #[inline]
    fn inv(self) -> Self {
        if self == 0.0 {
            panic!("inverse of zero (f64)");
        }
        1.0 / self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    type F7 = Fp<7>;

    #[test]
    fn fp_basic_ops() {
        let a = F7::new(3);
        let b = F7::new(5);
        assert_eq!((a + b).value(), 1);
        assert_eq!((a * b).value(), 1);
        assert_eq!((a - b).value(), 5);
        assert_eq!((a / b).value(), (a * b.inv()).value());
    }
}
