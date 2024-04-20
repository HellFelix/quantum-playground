use std::f64::consts::PI;

use super::{
    iteration::{
        descrete_derivative_matrix, descrete_potential_matrix, rk4_iter_dt, rk4_matrix_mul,
    },
    v, wave, DT, H_BAR,
};
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

// iteration testing
#[test]
#[allow(non_snake_case)]
fn oneD_iter() {
    let wave0 = wave();

    // iteration with matrix multiplication is reliable but horribly slow
    // Serves as a good test reference for the faster vector iteration rk4 method
    // that is trickier to get right.
    let size = wave0.0.len();
    let T = descrete_derivative_matrix(size);

    let V = descrete_potential_matrix(Box::new(v));

    let U = (DT / Complex::new(0., H_BAR)) * (&T + &V);
    let iter_matrix = rk4_matrix_mul(&wave0.1, &U);

    let iter_vector = rk4_iter_dt(&wave0.1);

    for i in 0..size {
        // the resulting values should be equal (with some leeway for floating point errors)
        assert!(iter_matrix[i].abs_squared() - iter_vector[i].abs_squared() < 1e-15)
    }
}
