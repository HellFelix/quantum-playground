use crate::{complex::*, iterate_pde_rk4, simpsons_rule, wave_n, DT, L};

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
