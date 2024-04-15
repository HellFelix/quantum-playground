use bevy::log::info;
use std::f64::consts::PI;

const H: f64 = 1.;
const H_BAR: f64 = H / (2. * PI);
const M: f64 = 1.;

const L: f64 = 1.;

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
    if x >= -L / 2. && x <= L / 2. {
        0.
    } else {
        f64::INFINITY
    }
}

fn e(n: usize) -> f64 {
    (n as f64).powi(2) * H.powi(2) / (8. * M * L.powi(2))
}

fn wave_n(n: usize) -> (Vec<f64>, Vec<Complex>) {
    let dx2 = |p, _dp, x| 2. * M * p * (v(x) - e(n)) / H_BAR.powi(2);

    let res = second_order_rk4(
        Complex::new(0., 0.),
        Complex::new(1., -1.),
        -L / 2.,
        L / 2.,
        DX,
        dx2,
    );

    (res.0, normalize(res.1))
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
    let k1 = |x_i| dphi_dt(x_i, &psi_n);
    let k2 = |x_i| dphi_dt(x_i, &psi_n) + 0.5 * h * k1(x_i);
    let k3 = |x_i| dphi_dt(x_i, &psi_n) + 0.5 * h * k2(x_i);
    let k4 = |x_i| dphi_dt(x_i, &psi_n) + h * k3(x_i);

    let psi_next_fn = |x_i| psi_n[x_i] + h / 6. * (k1(x_i) + 2. * k2(x_i) + 2. * k3(x_i) + k4(x_i));

    let mut psi_next: Vec<Complex> = Vec::new();
    // first value must always be the same bacause we cannot form a derivative at the start psi_n[-1] is invalid
    // this is ok because the edge values must always be zero since the potential barrier at that
    // those points are infinite
    psi_next.push(psi_n[0]);
    for i in 1..psi_n.len() - 1 {
        psi_next.push(psi_next_fn(i));
    }
    // last value is the same as before for the same reason as the first
    psi_next.push(*psi_n.last().unwrap());
    normalize(psi_next)
}

fn dphi_dt(x_i: usize, psi_n: &Vec<Complex>) -> Complex {
    let x_c = -L / 2. + x_i as f64 * DX;
    i() * ((psi_n[x_i + 1] - 2. * psi_n[x_i] + psi_n[x_i - 1]) / (2. * DX.powi(2))/* - v(x_c) * psi_n[x_i] */)
}
