#![allow(non_snake_case)]
use crate::{complex::*, H_BAR, L, M};
use nalgebra::{DMatrix, DVector};
use num_traits::{One, Zero};

use crate::DX;

pub fn descrete_derivative_matrix(size: usize) -> DMatrix<Complex> {
    let mut m = DMatrix::from_diagonal(&DVector::from(vec![(-2.).into(); size]));

    let mut ones = Vec::new();
    for i in 1..=size {
        let mut numbers = vec![Complex::zero(); size - 1];
        if i <= numbers.len() {
            numbers.insert(i, Complex::one());
        } else {
            numbers.push(Complex::zero());
        }
        ones.push(numbers);
    }
    let mut ones_matrix = DMatrix::from_vec(size, 1, ones[0].clone());
    ones.remove(0);
    for v in ones {
        ones_matrix.extend(v.clone());
    }

    m += ones_matrix.clone() + ones_matrix.transpose();

    -(H_BAR.powi(2) / (2. * M)) * Complex::from_real(1. / DX.powi(2)) * m
}

pub fn descrete_potential_matrix(v: Box<dyn Fn(f64) -> Complex>) -> DMatrix<Complex> {
    DMatrix::from_diagonal(&DVector::from(
        ((-L / (2. * DX)) as isize..=(L / (2. * DX)) as isize)
            .map(|x| v(x as f64 * DX))
            .collect::<Vec<Complex>>(),
    ))
}

pub fn rk4(psi0: &DVector<Complex>, U: &DMatrix<Complex>) -> DVector<Complex> {
    let k1 = U * psi0;
    let k2 = U * (psi0 + Complex::from_real(0.5) * &k1);
    let k3 = U * (psi0 + Complex::from_real(0.5) * &k2);
    let k4 = U * (psi0 + &k3);

    psi0 + Complex::from_real(1. / 6.)
        * &(k1 + Complex::from_real(2.) * &k2 + Complex::from_real(2.) * &k3 + &k4)
}
