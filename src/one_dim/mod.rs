// constants for simulation
const H: f64 = 1.;
const H_BAR: f64 = H / (2. * PI);
const M: f64 = 1.;
const L: f64 = 8.; //8

const DX: f64 = 0.01;
const DT: f64 = 0.0005;

// simulation specifics
const POTENTIAL: bool = false;

// External crates
use nalgebra::DVector;
use num_traits::Zero;
use std::f64::consts::{E, PI};

// internal modules
mod iteration;
mod visuals;
use crate::complex::*;
#[cfg(test)]
mod test;

pub fn run(visual: bool) {
    if visual {
        visuals::oneD();
    }
}

#[allow(unused_variables)]
fn v(x: f64) -> Complex {
    // if x > 2. && x < 2.5 {
    //     Complex::new(1., 0.)
    // } else {
    //     Complex::zero()
    // }
    Complex::zero()
}

// Creates a wave vector (vector containing the wave function's value at equally spaced
// x values) by assuming psi = int{c(k)e^(ikx)}dk, where c_k is a gaussian.
fn wave() -> (DVector<f64>, DVector<Complex>) {
    // the central value of c(k)
    let k_0: isize = 10;
    // we cannot integrate form -infty..infty, thus we make the cut-off at this value
    let k_range: isize = 10; // 10
                             // width of the gaussian
    let dk = 5f64;
    // c(k) = e^(-(k-k_0)/dk)^2
    let c_k = |k: f64| E.powf(-((k - k_0 as f64) / dk).powi(2));
    // psi_n(x, k) = e^(ikx)
    let f_k = |x: f64, k: f64| Complex::exp(i() * (k * x));

    let x_values: Vec<f64> = ((-L / (2. * DX)) as isize..=(L / (2. * DX)) as isize)
        .map(|x| x as f64 * DX)
        .collect();

    let mut wave: Vec<Complex> = Vec::new();
    for x in x_values.clone() {
        let mut p_x = Complex::zero();
        for k in (k_0 - k_range)..=(k_0 + k_range) {
            p_x += c_k(k as f64) * f_k(x, k as f64);
        }

        wave.push(p_x);
    }

    (DVector::from(x_values), DVector::from(normalize(wave)))
}

// Assuming equally spaced points, using simpsons rule makes it so the square of the
// inputted data itegrates to 1.
fn normalize(data: Vec<Complex>) -> Vec<Complex> {
    let data_squared = data.clone().iter().map(|x| x.abs_squared()).collect();
    let tot_integral = simpsons_rule(data_squared, -L / 2., L / 2.);
    // Each value must be devided by the root of the total integral since
    // we're considering the squares of each data point.
    data.iter().map(|x| *x / tot_integral.sqrt()).collect()
}

// assumes that the inputted data-points are equally spaced in terms of the independant variable
// and that the data starts at "start" and ends at "stop"
fn simpsons_rule(data: Vec<f64>, lower_bound: f64, upper_bound: f64) -> f64 {
    let mut sum: f64 = (1..data.len())
        .map(|i| {
            if i % 3 == 0 {
                2. * data[i]
            } else {
                3. * data[i]
            }
        })
        .sum();
    sum += data[0] + data.last().unwrap();

    sum * 3. * ((upper_bound - lower_bound) / data.len() as f64) / 8.
}
