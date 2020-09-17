use std::ops::Range;

pub struct Fern {
    pub size: f64,
    pub growth_rate: f64
}

impl Fern {
    // シダ植物の1日の成長をシミュレート
    pub fn grow(&mut self) {
        self.size *= 1.0 + self.growth_rate;
    }
}

pub fn run_simuration(fern: &mut Fern, days: usize) {
    for _ in 0 .. days {
        fern.grow();
    }
}

///
/// Return true if two ranges overlap.
///
///     assert_eq!(fern_sim::overlap(0..7, 3..10), true);
///     assert_eq!(fern_sim::overlap(1..5, 101..105), false);
pub fn overlap(r1: Range<usize>, r2: Range<usize>) -> bool {
    r1.start < r1.end && r2.start < r1.end &&
        r1.start < r2.end && r2.start < r1.end
}
