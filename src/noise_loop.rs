extern crate noise;
extern crate rand;

use rand::Rng;

use noise::{NoiseFn, Perlin};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy)]
pub struct NoiseLoop {
    pub diameter: f64,
    min: f64,
    max: f64,
    cx: f64,
    cy: f64,
}

impl NoiseLoop {
    pub fn new(diameter: f64, min: f64, max: f64) -> NoiseLoop {
        let mut rng = rand::thread_rng();

        NoiseLoop {
            diameter,
            min,
            max,
            cx: rng.gen_range(0., 1_000.),
            cy: rng.gen_range(0., 1_000.),
        }
    }

    pub fn value(&self, val: f64) -> f64 {
        let perlin = Perlin::new();

        let xoff = map((val).cos(), -1., 1., self.cx, self.cx + self.diameter);
        let yoff = map((val).sin(), -1., 1., self.cy, self.cy + self.diameter);
        map(perlin.get([xoff, yoff]), 0., 1., self.min, self.max)
    }
}

pub fn map<T>(val: T, or_start: T, or_fin: T, new_start: T, new_fin: T) -> T
where
    T: Div<Output = T> + Sub<Output = T> + Mul<Output = T> + Add<Output = T> + Clone + Copy,
{
    ((val - or_start) / (or_fin - or_start)) * (new_fin - new_start) + new_start
}
