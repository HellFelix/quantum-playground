use bevy::log::info;
use std::time::SystemTime;

use crate::{complex::*, DT, H_BAR, M};
use nalgebra::{DMatrix, DVector, OMatrix, LU};
use num_traits::{One, Zero};

use crate::DX;

pub fn descrete_derivative_matrix(size: usize, dx: f64) -> DMatrix<Complex> {
    let mut m = DMatrix::from_diagonal(&DVector::from(vec![Complex::new(-2., 0.); size]));

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

    -(H_BAR.powi(2) / (2. * M)) * Complex::new(1. / DX.powi(2), 0.) * m
}

pub fn descrete_potential_matrix(size: usize) -> DMatrix<Complex> {
    DMatrix::from_diagonal(&DVector::from(vec![Complex::zero(); size]))
}

pub fn rk4(psi0: DVector<Complex>, U: &DMatrix<Complex>) -> DVector<Complex> {
    let start_time = SystemTime::now();

    let k1 = U * &psi0;
    info!("mul took: {}", start_time.elapsed().unwrap().as_micros());

    let k2 = U * (&psi0 + Complex::new(0.5, 0.) * k1.clone());
    let k3 = U * (&psi0 + Complex::new(0.5, 0.) * k2.clone());
    let k4 = U * (&psi0 + k3.clone());

    psi0 + Complex::new(1. / 6., 0.)
        * (k1 + Complex::new(2., 0.) * k2 + Complex::new(2., 0.) * k3 + k4)
}
