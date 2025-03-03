use std::{
    f64::consts::E,
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

use nalgebra::{base::dimension::Dyn, base::VecStorage, Const, Matrix};
use num_traits::identities::{One, Zero};

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

    pub fn from_real(value: f64) -> Self {
        Self { re: value, im: 0. }
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

    pub fn exp(input: Self) -> Self {
        E.powf(input.re)
            * Self {
                re: (input.imag()).cos(),
                im: (input.imag()).sin(),
            }
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

// Shorthand
impl From<f64> for Complex {
    fn from(value: f64) -> Self {
        // assume value is real
        Self::new(value, 0.)
    }
}

// num_traits implementations
impl Zero for Complex {
    fn zero() -> Self {
        Self { re: 0., im: 0. }
    }
    fn is_zero(&self) -> bool {
        *self == Self { re: 0., im: 0. }
    }
}

// assume real
impl One for Complex {
    fn one() -> Self {
        Self { re: 1., im: 0. }
    }
    fn is_one(&self) -> bool
    where
        Self: PartialEq,
    {
        *self == Self { re: 1., im: 0. }
    }
}

// Matrix implementations
impl Mul<Matrix<Complex, Dyn, Dyn, VecStorage<Complex, Dyn, Dyn>>> for Complex {
    type Output = Matrix<Complex, Dyn, Dyn, VecStorage<Complex, Dyn, Dyn>>;
    fn mul(self, rhs: Matrix<Complex, Dyn, Dyn, VecStorage<Complex, Dyn, Dyn>>) -> Self::Output {
        let mut res = rhs.to_owned();

        for rhs in res.as_mut_slice().iter_mut() {
            *rhs *= self
        }

        res
    }
}

impl Mul<&Matrix<Complex, Dyn, Const<1>, VecStorage<Complex, Dyn, Const<1>>>> for Complex {
    type Output = Matrix<Complex, Dyn, Const<1>, VecStorage<Complex, Dyn, Const<1>>>;
    fn mul(
        self,
        rhs: &Matrix<Complex, Dyn, Const<1>, VecStorage<Complex, Dyn, Const<1>>>,
    ) -> Self::Output {
        let mut res = rhs.to_owned();

        for rhs in res.as_mut_slice().iter_mut() {
            *rhs *= self
        }

        res
    }
}

impl Mul<Matrix<Complex, Dyn, Const<1>, VecStorage<Complex, Dyn, Const<1>>>> for Complex {
    type Output = Matrix<Complex, Dyn, Const<1>, VecStorage<Complex, Dyn, Const<1>>>;
    fn mul(
        self,
        rhs: Matrix<Complex, Dyn, Const<1>, VecStorage<Complex, Dyn, Const<1>>>,
    ) -> Self::Output {
        let mut res = rhs.to_owned();

        for rhs in res.as_mut_slice().iter_mut() {
            *rhs *= self
        }

        res
    }
}
