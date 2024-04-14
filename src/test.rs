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
}

#[test]
fn f64_complex_arithmetic() {
    assert_eq!((2. - 5. * i()), Complex::new(2., -5.));
    assert_eq!((-4. + 9. * i()), Complex::new(-4., 9.));
    assert_eq!(3. * (1. - 2. * i()), Complex::new(3., -6.));
    assert_eq!(1. / (4. + 6. * i()), Complex::new(1. / 13., -3. / 26.));
}
