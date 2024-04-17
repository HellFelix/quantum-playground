use std::f64::consts::PI;

use crate::complex::*;

#[test]
fn basic_complex_arithmetic() {
    //addition (2-5i)+(-4+9i)
    assert_eq!((2. - 5. * i()) + (-4. + 9. * i()), -2. + 4. * i());
    assert_eq!((2. - 5. * i()) - (-4. + 9. * i()), 6. - 14. * i());
    assert_eq!((2. - 5. * i()) * (-4. + 9. * i()), 37. + 38. * i());
    assert_eq!(
        (2. - 5. * i()) / (-4. + 9. * i()),
        -53. / 97. + 2. / 97. * i()
    );

    assert_eq!(i() * i(), Complex::new(-1., 0.));
}

#[test]
fn f64_complex_arithmetic() {
    assert_eq!((2. - 5. * i()), Complex::new(2., -5.));
    assert_eq!((-4. + 9. * i()), Complex::new(-4., 9.));
    assert_eq!(3. * (1. - 2. * i()), Complex::new(3., -6.));
    assert_eq!(1. / (4. + 6. * i()), Complex::new(1. / 13., -3. / 26.));
}

#[test]
fn complex_display() {
    assert_eq!(format!("{}", 5. + 2. * i()), "5+2i");
    assert_eq!(format!("{}", 5. - 2. * i()), "5-2i");
}

#[test]
fn exponential() {
    assert!((Complex::exp(0. - i() * PI) - Complex::new(-1., 0.)).abs_squared() < 1e-30);
}

#[test]
fn assign() {
    let mut a = 2. - 5. * i();
    a += -4. + 9. * i();
    assert_eq!(a, -2. + 4. * i());

    let mut s = 2. - 5. * i();
    s -= -4. + 9. * i();
    assert_eq!(s, 6. - 14. * i());

    let mut m = 2. - 5. * i();
    m *= -4. + 9. * i();
    assert_eq!(m, 37. + 38. * i());

    let mut d = 2. - 5. * i();
    d /= -4. + 9. * i();
    assert_eq!(d, -53. / 97. + 2. / 97. * i());
}

// ----------------------------------------------------------------
// #[test]
// fn oneD_iteration() {
//     let mut wave = wave_n(1).1;
//     for _ in 0..20 {
//         wave = iterate_pde_rk4(wave, DT);
//         assert!((tot_prob(wave.clone()) - 1.).abs() < 1e-6);
//     }
// }
// // helper functions
// fn tot_prob(data: Vec<Complex>) -> f64 {
//     let data_squared = data.clone().iter().map(|x| x.abs_squared()).collect();
//     simpsons_rule(data_squared, -L / 2., L / 2.)
// }
