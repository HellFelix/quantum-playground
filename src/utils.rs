// assumes that the inputted data-points are equally spaced in terms of the independant variable
// and that the data starts at "start" and ends at "stop"
pub fn simpsons_rule(data: Vec<f64>, lower_bound: f64, upper_bound: f64) -> f64 {
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
