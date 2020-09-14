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
