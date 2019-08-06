// Draw the classic triangle to the screen
extern crate noise;
extern crate quicksilver;
extern crate rand;

mod noise_loop;
use noise_loop::map;

use quicksilver::{
    geom::{Circle, Vector},
    graphics::{Background::Col, Color, GpuTriangle, Mesh, Vertex},
    input::{ButtonState, Key, MouseButton},
    lifecycle::{run, Event, Settings, State, Window},
    Result,
};

const WIDTH: f32 = 700.;
const HEIGHT: f32 = 500.;

struct MainState {
    point: Vec<(f32, f32)>,
    counter: f64,
}

impl State for MainState {
    fn new() -> Result<MainState> {
        Ok(MainState {
            point: vec![(0., 0.)],
            counter: 0.,
        })
    }

    fn event(&mut self, event: &Event, window: &mut Window) -> Result<()> {
        match *event {
            Event::MouseButton(MouseButton::Left, ButtonState::Pressed) => {
                self.point = Vec::new();
            }
            Event::Key(Key::Escape, ButtonState::Pressed) => {
                window.close();
            }
            _ => (),
        }
        Ok(())
    }

    fn update(&mut self, _window: &mut Window) -> Result<()> {
        if self.point.len() < 3200 {
            for _ in 0..10 {
                let (x, y) = self.point[self.point.len() - 1];
                let val = rand::random::<f64>();
                match val {
                    _ if val < 0.01 => {
                        let next_x = 0.;
                        let next_y = y * 0.16;
                        self.point.push((next_x, next_y))
                    }
                    _ if val < 0.86 => {
                        let next_x = (0.85 * x) + (0.04 * y);
                        let next_y = (-0.04 * x) + (0.85 * y) + 1.6;
                        self.point.push((next_x, next_y))
                    }
                    _ if val < 0.93 => {
                        let next_x = (0.2 * x) - (0.26 * y);
                        let next_y = (0.23 * x) + (0.22 * y) + 1.6;
                        self.point.push((next_x, next_y))
                    }
                    _ if val < 1. => {
                        let next_x = (-0.15 * x) + (0.28 * y);
                        let next_y = (0.26 * x) + (0.24 * y) + 0.44;
                        self.point.push((next_x, next_y))
                    }
                    _ => (),
                };
                self.counter += 1.;
            }
        }
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::BLACK)?;
        for p in self.point.iter() {
            let (mut x, mut y) = p;
            x = map(x, -2.1820, 2.6558, 0., WIDTH);
            y = map(y, 0., 9.9983, HEIGHT, 0.);

            window.draw(&Circle::new((x, y), 2.), Col(Color::WHITE));
        }
        Ok(())
    }
}

fn main() {
    run::<MainState>("Barnsley fern", Vector::new(800, 600), Settings::default());
}
