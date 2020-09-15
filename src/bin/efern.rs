extern crate fern_sim;
use fern_sim::{Fern, run_simuration};

fn main() {
    let mut fern = Fern {
        size: 1.0,
        growth_rate: 0.001
    };
    run_simuration(&mut fern, 1000);
    println!("final fern size: {}", fern.size);
}

fn rouchly_equal(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-6
}

#[test]
fn trig_works() {
    use std::f64::consts::PI;
    assert!(rouchly_equal(PI.sin(), 0.0));
}
