use std::{f64::consts::E, vec};

use nalgebra::DVector;
use num_traits::Zero;

use crate::{complex::{i, Complex}, utils::simpsons_rule};

const L: f64 = 8.;
const DL: f64 = 0.1;

mod visuals;

pub fn run(visual: bool) {
    if visual {
        visuals::twoD();
    }
}

pub fn wave() -> DVector<DVector<(f64, Complex, f64)>> {
        // the central value of c(k)
    let k_0: isize = 10;
    // we cannot integrate form -infty..infty, thus we make the cut-off at this value
    let k_range: isize = 5; // 10
    let dk = 0.5; // discretesation of the grid of k
    let k_values = ((k_0 - k_range)..=(k_0 + k_range))
        .map(|x| x as f64 * dk)
        .collect::<Vec<f64>>();

    let delta_k = 5f64; // width of the gaussian
                        // c(k) = e^(-(k-k_0)/dk)^2
    let c_k = |k: f64| E.powf(-((k - k_0 as f64) / delta_k).powi(2));
    // psi_n(x, k) = e^(ikx)
    let f_k = |u: f64, k: f64| Complex::exp(i() * (k * u));

    // General gaussian
    // let f = |x: f32, z: f32| {E.powf(-(x.powi(2)+z.powi(2)))};
    let mut wave: DVector<DVector<(f64, Complex, f64)>> = DVector::from(vec![]);

    for i in -(L/(DL*2.)) as isize..=(L/(DL*2.)) as isize {
        let x_c = i as f64 *DL;
        let mut ind_wave = vec![];
        for j in -(L/(DL*2.)) as isize..=(L/(DL*2.)) as isize {
            let z_c = j as f64 * DL;
            let mut res = Complex::zero();
            for k_x in &k_values {
                for k_z in &k_values {
                    res += c_k(*k_x)*c_k(*k_z) * f_k(x_c, *k_x) * f_k(z_c, *k_z);
                }
            }
            ind_wave.push((x_c, res, z_c));
        }
        wave = wave.push(ind_wave.into());
    }

    normalize_2d(wave)
}

// double integral for 2d
fn normalize_2d(mut data: DVector<DVector<(f64, Complex, f64)>>) -> DVector<DVector<(f64, Complex, f64)>> {
    let mut int_vec = Vec::new();
    // integrate for every vector by itself
    for v in data.clone().iter() {
        let data_squared = v.clone().iter().map(|x| x.1.abs_squared()).collect::<Vec<f64>>();
        int_vec.push(simpsons_rule(data_squared, -L/2., L/2.));
    }

    // integrate for the combined integral
    let total_integral = simpsons_rule(int_vec, -L/2., L/2.);
    println!("{total_integral}");
    let int_factor = 1./total_integral.sqrt();

    for v in &mut data {
        v.iter_mut().for_each(|x| x.1 *= int_factor);
    }

    data
}

// pub fn dt(psi0: &DVector<DVector<Complex>>) -> DVector<DVector<Complex>> {
//     let k1 = d_dt(&psi0);
//     let k2 = d_dt(&(psi0 + Complex::from_real(0.5) * &k1));
//     let k3 = d_dt(&(psi0 + Complex::from_real(0.5) * &k2));
//     let k4 = d_dt(&(psi0 + &k3));

//     psi0 + Complex::from_real(1. / 6.)
//         * &(k1 + Complex::from_real(2.) * &k2 + Complex::from_real(2.) * &k3 + &k4)
// }

// fn d_dt(f: &DVector<DVector<Complex>>) -> DVector<DVector<Complex>> {
//     unimplemented!()
// }
