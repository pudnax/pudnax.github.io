#![allow(dead_code)]
extern crate quicksilver;

extern crate rand;
use rand::{thread_rng, Rng};

// use ggez::graphics::{self, DrawParam, MeshBuilder};
// use ggez::{Context, GameResult};

use quicksilver::{
    geom::Line,
    graphics::{Background::Col, Color},
    lifecycle::Window,
    Result,
};

use super::vect2d::Vect2D;

#[derive(Copy, Clone, Debug)]
pub struct Boundary {
    pub a: Vect2D,
    pub b: Vect2D,
}

impl Boundary {
    pub fn new(x1: f32, y1: f32, x2: f32, y2: f32) -> Boundary {
        Boundary {
            a: Vect2D::new(x1, y1),
            b: Vect2D::new(x2, y2),
        }
    }

    pub fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.draw(
            &Line::new((self.a.x, self.a.y), (self.b.x, self.b.y)).with_thickness(2.),
            Col(Color::WHITE),
        );

        Ok(())
    }

    pub fn random(x: f32, y: f32) -> Boundary {
        let mut rng = thread_rng();
        let w1: f32 = rng.gen_range(0., x);
        let w2: f32 = rng.gen_range(0., x);
        let h1: f32 = rng.gen_range(0., y);
        let h2: f32 = rng.gen_range(0., y);

        Boundary::new(w1, h1, w2, h2)
    }
}
