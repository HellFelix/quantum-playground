#![allow(non_snake_case)]
use super::{v, DT, H_BAR, L, M, POTENTIAL};
use crate::complex::*;
use nalgebra::{DMatrix, DVector};
use num_traits::{One, Zero};

use super::DX;
pub fn rk4_iter_dt(psi0: &DVector<Complex>) -> DVector<Complex> {
    let k1 = d_dt(&psi0);
    let k2 = d_dt(&(psi0 + Complex::from_real(0.5) * &k1));
    let k3 = d_dt(&(psi0 + Complex::from_real(0.5) * &k2));
    let k4 = d_dt(&(psi0 + &k3));

    psi0 + Complex::from_real(1. / 6.)
        * &(k1 + Complex::from_real(2.) * &k2 + Complex::from_real(2.) * &k3 + &k4)
}

pub fn d_dt(f: &DVector<Complex>) -> DVector<Complex> {
    let f_h = -(H_BAR.powi(2) / (2. * M)) * Complex::from_real(1. / DX.powi(2)) * f;
    let last = f.len() - 1;
    let mut pre_deriv = vec![-2. * f_h[0] + f_h[1]];
    for i in 1..last {
        pre_deriv.push(f_h[i - 1] - 2. * f_h[i] + f_h[i + 1]);
    }
    pre_deriv.push(f_h[last - 1] + -2. * f_h[last]);
    let deriv = DVector::from(pre_deriv);

    if POTENTIAL {
        let potent = &mut DVector::from(
            ((-L / (2. * DX)) as isize..=(L / (2. * DX)) as isize)
                .map(|x| v(x as f64 * DX))
                .collect::<Vec<Complex>>(),
        );
        for i in 0..f.len() {
            potent[i] *= f[i];
        }
        (DT / Complex::new(0., H_BAR)) * (deriv + potent.to_owned())
    } else {
        // if there is no potential at all, there is no reason to
        // calculate the potential vector either
        (DT / Complex::new(0., H_BAR)) * deriv
    }
}

// Using matrix multiplication is a lot more elegant, and easier to get right.
// The time trade-off is not worth it though. Doing the calculations without
// matricies saves time of more than two orders of magnitude for larger L values.
// This is because when increasing L, the traditional calculation grows linearly O(n)
// while the matrix multiplication grows quadratically O(n^2).
//
// The matrix functions are kept because they serve as a nice test for the faster rk4_iter_dt
// function
#[allow(dead_code)]
pub fn rk4_matrix_mul(psi0: &DVector<Complex>, U: &DMatrix<Complex>) -> DVector<Complex> {
    let k1 = U * psi0;
    let k2 = U * (psi0 + Complex::from_real(0.5) * &k1);
    let k3 = U * (psi0 + Complex::from_real(0.5) * &k2);
    let k4 = U * (psi0 + &k3);

    psi0 + Complex::from_real(1. / 6.)
        * &(k1 + Complex::from_real(2.) * &k2 + Complex::from_real(2.) * &k3 + &k4)
}

// helper functions for rk4_matrix_mul
#[allow(dead_code)]
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

#[allow(dead_code)]
pub fn descrete_potential_matrix(v: Box<dyn Fn(f64) -> Complex>) -> DMatrix<Complex> {
    DMatrix::from_diagonal(&DVector::from(
        ((-L / (2. * DX)) as isize..=(L / (2. * DX)) as isize)
            .map(|x| v(x as f64 * DX))
            .collect::<Vec<Complex>>(),
    ))
}
