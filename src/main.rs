use bevy::{log::info, render::render_resource::encase::private::RuntimeSizedArray};
use std::f64::consts::{E, PI};

const H: f64 = 1.;
const H_BAR: f64 = H / (2. * PI);
const M: f64 = 1.;

const L: f64 = 8.;

const DX: f64 = 0.01;
const DT: f64 = 0.01;

#[cfg(test)]
mod test;

mod complex;
use complex::*;

mod visuals;
const VISUAL: bool = true;

fn main() {
    if VISUAL {
        visuals::oneD();
    }
}

fn v(x: f64) -> f64 {
    0.
}

fn e(n: usize) -> f64 {
    (n as f64).powi(2) * H.powi(2) / (8. * M * L.powi(2))
}

fn wave() -> (Vec<f64>, Vec<Complex>) {
    let k_0: isize = 10;
    let k_range: isize = 10; // 20
    let dk = 5f64;
    let c_k = |k: f64| E.powf(-((k - k_0 as f64) / dk).powi(2));
    let f_k = |x: f64, k: f64| (k * x).cos() + i() * (k * x).sin();

    let x_values: Vec<f64> = ((-L / (2. * DX)) as isize..=(L / (2. * DX)) as isize)
        .map(|x| x as f64 * DX)
        .collect();

    let mut wave: Vec<Complex> = Vec::new();
    for x in x_values.clone() {
        let mut p_x = Complex::new(0., 0.);
        for k in (k_0 - k_range)..=(k_0 + k_range) {
            p_x += c_k(k as f64) * f_k(x, k as f64);
        }

        wave.push(p_x);
    }

    (x_values, normalize(wave))
}

fn normalize(data: Vec<Complex>) -> Vec<Complex> {
    let data_squared = data.clone().iter().map(|x| x.abs_squared()).collect();
    let tot_integral = simpsons_rule(data_squared, -L / 2., L / 2.);
    data.iter().map(|x| *x / tot_integral.sqrt()).collect()
}

pub fn second_order_rk4(
    y0: Complex,
    dy0: Complex,
    x0: f64,
    xe: f64,
    h: f64,
    // f(y, dy, x)
    f: impl Fn(Complex, Complex, f64) -> Complex,
) -> (Vec<f64>, Vec<Complex>) {
    // value vectors
    let x_n: Vec<f64> = ((x0 / h) as isize..=(xe / h) as isize)
        .map(|x| x as f64 * h)
        .collect();
    let mut y_n: Vec<Complex> = Vec::new();

    // initial conditions
    let mut y = y0;
    let mut dy = dy0;

    for x in x_n.clone() {
        y_n.push(y);

        let l1 = h * dy;
        let k1 = h * f(y, dy, x);

        let l2 = h * (dy + 0.5 * k1);
        let k2 = h * f(y + 0.5 * l1, dy + 0.5 * k1, x + 0.5 * h);

        let l3 = h * (dy + 0.5 * k2);
        let k3 = h * f(y + 0.5 * l2, dy + 0.5 * k2, x + 0.5 * h);

        let l4 = h * (dy + k3);
        let k4 = h * f(y + l3, dy + k3, x + h);

        y += (l1 + 2. * l2 + 2. * l3 + l4) / 6.;
        dy += (k1 + 2. * k2 + 2. * k3 + k4) / 6.;
    }

    (x_n, y_n)
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

fn iterate_pde_rk4(psi_n: Vec<Complex>, h: f64) -> Vec<Complex> {
    let last = psi_n.len() - 1;
    let k1 = |x_i| dphi_dt(x_i, &psi_n, last);
    let k2 = |x_i| dphi_dt(x_i, &psi_n, last) + 0.5 * h * k1(x_i);
    let k3 = |x_i| dphi_dt(x_i, &psi_n, last) + 0.5 * h * k2(x_i);
    let k4 = |x_i| dphi_dt(x_i, &psi_n, last) + h * k3(x_i);

    let psi_next_fn = |x_i| psi_n[x_i] + h / 6. * (k1(x_i) + 2. * k2(x_i) + 2. * k3(x_i) + k4(x_i));

    let mut psi_next: Vec<Complex> = Vec::new();
    for i in 0..psi_n.len() {
        psi_next.push(psi_next_fn(i));
    }
    normalize(psi_next)
}

fn dphi_dt(x_i: usize, psi_n: &Vec<Complex>, last: usize) -> Complex {
    let x_c = -L / 2. + x_i as f64 * DX;

    if x_i == 0 {
        // foreward derivative
        i() * ((psi_n[x_i + 2] - 2. * psi_n[x_i + 1] + psi_n[x_i]) / DX.powi(2)
            - v(x_c) * psi_n[x_i])
    } else if x_i == last {
        i() * ((psi_n[x_i] - 2. * psi_n[x_i - 1] + psi_n[x_i - 2]) / DX.powi(2)
            - v(x_c) * psi_n[x_i])
    } else {
        i() * ((psi_n[x_i + 1] - 2. * psi_n[x_i] + psi_n[x_i - 1]) / DX.powi(2)
            - v(x_c) * psi_n[x_i])
    }
}
