#[derive(Clone, Copy)]
pub struct Angle {
    val: f64,
    step: f64,
}

impl Angle {
    pub fn new(step: f64) -> Angle {
        Angle {
            val: 2. * std::f64::consts::PI,
            step,
        }
    }
}

impl Iterator for Angle {
    type Item = f64;

    fn next(&mut self) -> Option<f64> {
        self.val -= self.step;
        if self.val < 0. {
            self.val = 2. * std::f64::consts::PI;
            None
        } else {
            Some(self.val)
        }
    }
}
