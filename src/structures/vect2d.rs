#![allow(dead_code)]
extern crate rand;
use rand::{thread_rng, Rng};
use std::cmp;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Copy, Clone)]
pub struct Vect2D {
    pub x: f32,
    pub y: f32,
}

impl Vect2D {
    fn eucl(x: f32, y: f32) -> f32 {
        (x * x + y * y).sqrt()
    }

    pub fn len(&self) -> f32 {
        Vect2D::eucl(self.x, self.y)
    }

    pub fn new(x: f32, y: f32) -> Vect2D {
        Vect2D { x, y }
    }

    pub fn zero() -> Vect2D {
        Vect2D::new(1., 1.)
    }

    pub fn scale(&self, scalar: f32) -> Vect2D {
        Vect2D::new(self.x * scalar, self.y * scalar)
    }

    pub fn lerp(v1: &Vect2D, v2: &Vect2D, alpha: f32) -> Vect2D {
        Vect2D::new(v1.x + (v2.x - v1.x) * alpha, v1.y + (v2.y - v1.y) * alpha)
    }

    pub fn clamp(&self, min: f32, max: f32) -> Vect2D {
        Vect2D::new(self.x.max(min).min(max), self.y.max(min).min(max))
    }

    /// Generates a random vector across a uniform distribution using the answer found in
    /// http://stackoverflow.com/questions/5408276/python-uniform-spherical-distribution
    pub fn random() -> Vect2D {
        let mut rng = thread_rng();
        let phi: f32 = rng.gen_range(0.0, 2.0 * ::std::f32::consts::PI);
        let costheta: f32 = rng.gen_range(-1.0, 1.0);
        let u: f32 = rng.gen_range(0.0, 1.0);

        let theta = costheta.acos();
        let r = u.powf(1.0 / 3.0);

        Vect2D::new(r * theta.sin() * phi.cos(), r * theta.sin() * phi.sin())
    }

    pub fn normalize(&mut self) {
        let norm = Vect2D::eucl(self.x, self.y);
        self.x /= norm;
        self.y /= norm;
    }

    pub fn dot(&self, vec: Vect2D) -> f32 {
        self.x * vec.x + self.y * vec.y
    }

    /// Create a unit vector representing the
    /// given angle (in radians)
    fn vec_from_angle(angle: f32) -> Vect2D {
        let vx = angle.sin();
        let vy = angle.cos();
        Vect2D::new(vx, vy)
    }
}

impl Add for Vect2D {
    type Output = Vect2D;

    fn add(self, other: Vect2D) -> Vect2D {
        Vect2D::new(self.x + other.x, self.y + other.y)
    }
}

impl Add<f32> for Vect2D {
    type Output = Vect2D;

    fn add(self, other: f32) -> Vect2D {
        Vect2D::new(self.x + other, self.y + other)
    }
}

impl Sub for Vect2D {
    type Output = Vect2D;

    fn sub(self, other: Vect2D) -> Vect2D {
        Vect2D::new(self.x - other.x, self.y - other.y)
    }
}

impl Sub<f32> for Vect2D {
    type Output = Vect2D;

    fn sub(self, other: f32) -> Vect2D {
        Vect2D::new(self.x - other, self.y - other)
    }
}

impl Mul for Vect2D {
    type Output = Vect2D;

    fn mul(self, other: Vect2D) -> Vect2D {
        Vect2D::new(self.x * other.x, self.y * other.y)
    }
}

impl Mul<f32> for Vect2D {
    type Output = Vect2D;

    fn mul(self, other: f32) -> Vect2D {
        Vect2D::new(self.x * other, self.y * other)
    }
}

impl Div for Vect2D {
    type Output = Vect2D;

    fn div(self, other: Vect2D) -> Vect2D {
        Vect2D::new(self.x / other.x, self.y / other.y)
    }
}

impl Div<f32> for Vect2D {
    type Output = Vect2D;

    fn div(self, other: f32) -> Vect2D {
        Vect2D::new(self.x / other, self.y / other)
    }
}

impl Neg for Vect2D {
    type Output = Vect2D;

    fn neg(self) -> Vect2D {
        Vect2D::new(-self.x, -self.y)
    }
}

impl cmp::PartialEq for Vect2D {
    fn eq(&self, other: &Vect2D) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl fmt::Debug for Vect2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
