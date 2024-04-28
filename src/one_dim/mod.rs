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
use crate::utils::simpsons_rule;
mod iteration;
mod visuals;
use crate::complex::{Complex, *};
#[cfg(test)]
mod test;

pub fn run(visual: bool) {
    if visual {
        visuals::oneD();
    }
}

fn barriers() -> Vec<Box<dyn Fn(f64) -> Complex>> {
    vec![
        // rectangle shaped barrier between x = 2.5 and x = 3
        Box::new(|x| {
            if x > 2.5 && x < 3. {
                Complex::from_real(1.)
            } else {
                Complex::zero()
            }
        }),
        // quadratic potential based around zero
        Box::new(|x| Complex::from_real(x.powi(2))),
    ]
}
fn v(x: f64) -> Complex {
    let mut res = Complex::zero();
    for b in barriers() {
        res += b(x);
    }
    res
}

// Creates a wave vector (vector containing the wave function's value at equally spaced
// x values) by assuming psi = int{c(k)e^(ikx)}dk, where c_k is a gaussian.
fn wave() -> (DVector<f64>, DVector<Complex>) {
    // the central value of c(k)
    let k_0: isize = 10;
    // we cannot integrate form -infty..infty, thus we make the cut-off at this value
    let k_range: isize = 10; // 10
    let dk = 0.5; // discretesation of the grid of k
    let k_values = ((k_0 - k_range)..=(k_0 + k_range))
        .map(|x| x as f64 * dk)
        .collect::<Vec<f64>>();

    let delta_k = 5f64; // width of the gaussian

    // c(k) = e^(-(k-k_0)/dk)^2
    let c_k = |k: f64| E.powf(-((k - k_0 as f64) / delta_k).powi(2));
    // psi_n(x, k) = e^(ikx)
    let f_k = |x: f64, k: f64| Complex::exp(i() * (k * x));

    let x_values: Vec<f64> = ((-L / (2. * DX)) as isize..=(L / (2. * DX)) as isize)
        .map(|x| x as f64 * DX)
        .collect();

    let mut wave: Vec<Complex> = Vec::new();
    for x in x_values.clone() {
        let mut p_x = Complex::zero();
        for k in &k_values {
            p_x += c_k(*k) * f_k(x, *k);
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
