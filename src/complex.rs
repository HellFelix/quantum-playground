use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex {
    re: f64,
    im: f64,
}
impl Complex {
    pub fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }
    pub fn complex_conjugate(&self) -> Self {
        Self {
            re: self.re,
            im: -self.im,
        }
    }
    pub fn from_vecf64(input: Vec<f64>) -> Vec<Self> {
        input.iter().map(|x| Self { re: *x, im: 0. }).collect()
    }

    pub fn real(&self) -> f64 {
        self.re
    }
    pub fn imag(&self) -> f64 {
        self.im
    }

    pub fn abs_squared(&self) -> f64 {
        self.re.powi(2) + self.im.powi(2)
    }
}

pub fn i() -> Complex {
    Complex { re: 0., im: 1. }
}

impl Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.im < 0. {
            write!(f, "{}{}i", self.re, self.im)
        } else {
            write!(f, "{}+{}i", self.re, self.im)
        }
    }
}

// basic arithmetic
impl Add<Self> for Complex {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}
impl Sub<Self> for Complex {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }
}
impl Mul<Self> for Complex {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }
}
impl Div<Self> for Complex {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        // (a+bi)/(c+di) = (a+bi)(c-di)/(c^2+d^2)
        let numerator = self * rhs.complex_conjugate();
        let denomenator = rhs.re.powi(2) + rhs.im.powi(2);

        Self::Output {
            re: numerator.re / denomenator,
            im: numerator.im / denomenator,
        }
    }
}

// for f64, assume real
impl Add<f64> for Complex {
    type Output = Self;
    fn add(self, rhs: f64) -> Self::Output {
        Self::Output {
            re: self.re + rhs,
            im: self.im,
        }
    }
}
impl Sub<f64> for Complex {
    type Output = Self;
    fn sub(self, rhs: f64) -> Self::Output {
        Self::Output {
            re: self.re - rhs,
            im: self.im,
        }
    }
}
impl Mul<f64> for Complex {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output {
            re: self.re * rhs,
            im: self.im * rhs,
        }
    }
}
impl Div<f64> for Complex {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self::Output {
            re: self.re / rhs,
            im: self.im / rhs,
        }
    }
}

impl Add<Complex> for f64 {
    type Output = Complex;
    fn add(self, rhs: Complex) -> Self::Output {
        Self::Output {
            re: rhs.re + self,
            im: rhs.im,
        }
    }
}
impl Sub<Complex> for f64 {
    type Output = Complex;
    fn sub(self, rhs: Complex) -> Self::Output {
        Self::Output {
            re: self - rhs.re,
            im: -rhs.im,
        }
    }
}
impl Mul<Complex> for f64 {
    type Output = Complex;
    fn mul(self, rhs: Complex) -> Self::Output {
        rhs * self
    }
}
impl Div<Complex> for f64 {
    type Output = Complex;
    fn div(self, rhs: Complex) -> Self::Output {
        Complex { re: self, im: 0. } / rhs
    }
}

// assign
impl AddAssign for Complex {
    fn add_assign(&mut self, rhs: Self) {
        let res = *self + rhs;
        *self = res;
    }
}
impl AddAssign<f64> for Complex {
    fn add_assign(&mut self, rhs: f64) {
        let res = *self + rhs;
        *self = res;
    }
}
impl SubAssign for Complex {
    fn sub_assign(&mut self, rhs: Self) {
        let res = *self - rhs;
        *self = res
    }
}
impl SubAssign<f64> for Complex {
    fn sub_assign(&mut self, rhs: f64) {
        let res = *self - rhs;
        *self = res
    }
}
impl MulAssign for Complex {
    fn mul_assign(&mut self, rhs: Self) {
        let res = *self * rhs;
        *self = res;
    }
}
impl MulAssign<f64> for Complex {
    fn mul_assign(&mut self, rhs: f64) {
        let res = *self * rhs;
        *self = res;
    }
}
impl DivAssign for Complex {
    fn div_assign(&mut self, rhs: Self) {
        let res = *self / rhs;
        *self = res;
    }
}
impl DivAssign<f64> for Complex {
    fn div_assign(&mut self, rhs: f64) {
        let res = *self / rhs;
        *self = res;
    }
}
