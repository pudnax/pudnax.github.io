#![allow(dead_code)]
extern crate quicksilver;

// use ggez::graphics::{self, DrawParam, MeshBuilder};
// use ggez::{Context, GameResult};

use quicksilver::{
    geom::Line,
    graphics::{Background::Col, Color},
    lifecycle::Window,
    Result,
};

use super::boundary::Boundary;
use super::vect2d::Vect2D;

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub pos: Vect2D,
    pub dir: Vect2D,
}

impl Ray {
    pub fn new(posx: f32, posy: f32, dirx: f32, diry: f32) -> Ray {
        Ray {
            pos: Vect2D::new(posx, posy),
            dir: Vect2D::new(posx + dirx * 10., posy + diry * 10.),
        }
    }

    pub fn look_at(&mut self, x: f32, y: f32) {
        self.dir.x = x - self.pos.x;
        self.dir.y = y - self.pos.y;
        self.dir.normalize();
    }

    pub fn cast(&self, wall: Boundary) -> Option<Vect2D> {
        let x1 = wall.a.x;
        let y1 = wall.a.y;
        let x2 = wall.b.x;
        let y2 = wall.b.y;
        let x3 = self.pos.x;
        let y3 = self.pos.y;
        let x4 = self.dir.x;
        let y4 = self.dir.y;

        let det = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
        if det == 0. {
            return None;
        }
        let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / det;
        let u = -((x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x3)) / det;
        //        println!("{} {}", t, u);
        if u >= 0. && t >= 0. && t <= 1. {
            let px = x1 + t * (x2 - x1);
            let py = y1 + t * (y2 - y1);
            //        let px = (self.pos.x * self.dir.y - self.pos.y * self.dir.x) * (wall.a.x - wall.b.x)
            //            - (self.pos.x - self.dir.x) * (wall.a.x * wall.b.y - wall.a.x * wall.b.x) / det;
            //        let py = (self.pos.x * self.dir.y - self.pos.y * self.dir.x) * (wall.b.x - wall.a.x)
            //            - (self.pos.y - self.dir.y) * (wall.a.x * wall.b.y - wall.a.x * wall.b.x) / det;
            Some(Vect2D::new(px, py))
        } else {
            None
        }
    }

    pub fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.draw(
            &Line::new(
                (self.pos.x, self.pos.y),
                (self.pos.x + self.dir.x, self.pos.y + self.dir.y),
            )
            .with_thickness(2.),
            Col(Color::WHITE),
        );
        Ok(())
    }
}
