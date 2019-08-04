extern crate noise;
extern crate quicksilver;
extern crate rand;

use noise::{NoiseFn, Perlin};

mod angle_iter;
mod noise_loop;

use angle_iter::Angle;
use noise_loop::map;

use quicksilver::{
    geom::{Line, Rectangle, Vector},
    graphics::{Background::Col, Color, Vertex, View},
    lifecycle::{run, Settings, State, Window},
    Result,
};

const WIDTH: f32 = 900.;
const HEIGHT: f32 = 800.;

struct MainState {
    angle: Angle,
    vertices: Vec<Vertex>,
    phase: f64,
    record: bool,
    counter: usize,
    total_frames: usize,
}

impl MainState {
    fn draw_contour(&mut self, window: &mut Window) {
        let size = self.vertices.len();
        for i in 0..size {
            window.draw(
                &Line::new(self.vertices[i].pos, self.vertices[(i + 1) % size].pos),
                Col(self.vertices[i].col),
            )
        }
    }
}

impl State for MainState {
    fn new() -> Result<MainState> {
        Ok(MainState {
            angle: Angle::new(0.2),
            vertices: Vec::new(),
            phase: 0.,
            record: true,
            counter: 0,
            total_frames: 600,
        })
    }

    fn update(&mut self, _window: &mut Window) -> Result<()> {
        self.vertices = Vec::new();

        for a in self.angle {
            let perlin = Perlin::new();
            let xoff = map((a + self.phase).cos(), -1., 1., 0., 0.7);
            let yoff = map((a - self.phase).sin(), -1., 1., 0., 0.7);
            let r = map(perlin.get([xoff, yoff]), 0., 1., 200., 400.);
            let x = r * a.cos();
            let y = r * a.sin();
            self.vertices
                .push(Vertex::new((x as f32, y as f32), None, Col(Color::WHITE)))
        }
        self.vertices.push(self.vertices[0]);
        self.phase = self.counter as f64 / self.total_frames as f64 * 2. * std::f64::consts::PI;
        self.counter += 1;
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::BLACK)?;
        window.set_view(View::new(Rectangle::new(
            (-WIDTH / 2., -HEIGHT / 2.),
            (WIDTH, HEIGHT),
        )));

        self.draw_contour(window);
        Ok(())
    }
}

fn main() {
    run::<MainState>(
        "Polar_perlin",
        Vector::new(WIDTH, HEIGHT),
        Settings::default(),
    );
}
