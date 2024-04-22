const L: f64 = 8.;

mod visuals;

pub fn run(visual: bool) {
    if visual {
        visuals::twoD();
    }
}
