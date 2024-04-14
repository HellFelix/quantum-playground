use std::f64::consts::PI;

const H: f64 = 1.;
const H_BAR: f64 = H / (2. * PI);
const M: f64 = 1.;

const L: f64 = 1.;

#[cfg(test)]
mod test;

mod complex;
use complex::Complex;

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

fn wave_n(n: usize) -> (Vec<f64>, Vec<f64>) {
    let dx2 = |p, _dp, x| 2. * M * p * (v(x) - e(n)) / H_BAR.powi(2);

    let res = second_order_rk4(0., 1., -L / 2., L / 2., 0.01, dx2);

    (res.0, normalize(res.1))
}

fn normalize(data: Vec<f64>) -> Vec<f64> {
    let data_squared = data.clone().iter().map(|x| x.abs().powi(2)).collect();
    let tot_integral = simpsons_rule(data_squared, -L / 2., L / 2.);
    data.iter().map(|x| x / tot_integral.sqrt()).collect()
}

pub fn second_order_rk4(
    y0: f64,
    dy0: f64,
    x0: f64,
    xe: f64,
    h: f64,
    // y, dy, x
    f: impl Fn(f64, f64, f64) -> f64,
) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    // value vectors
    let x_n: Vec<f64> = ((x0 / h) as isize..=(xe / h) as isize)
        .map(|x| x as f64 * h)
        .collect();
    let mut y_n: Vec<f64> = Vec::new();
    let mut dy_n: Vec<f64> = Vec::new();

    // initial conditions
    let mut y = y0;
    let mut dy = dy0;

    for x in x_n.clone() {
        y_n.push(y);
        dy_n.push(dy);

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

    (x_n, y_n, dy_n)
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

// fn pde_rk4(psi0: Vec<Complex>, t0: f64, te: f64, h: f64) -> (Vec<f64>, Vec<Vec<Complex>>) {
//     let dpsi = |x, t| {Complex::i()*(psi0[x-1])}

//     unimplemented!()
// }
