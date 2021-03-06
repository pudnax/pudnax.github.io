#![allow(dead_code)]
// Draw the classic triangle to the screen
extern crate quicksilver;
extern crate rand;

mod angle_iter;

use rand::Rng;

use quicksilver::{
    geom::{Circle, Line, Transform, Vector},
    graphics::{Background::Col, Color, GpuTriangle, Mesh, Vertex},
    input::{ButtonState, Key, MouseButton},
    lifecycle::{run, Event, Settings, State, Window},
    Result,
};

const WIDTH: f32 = 1024.;
const HEIGHT: f32 = 576.;

#[derive(Copy, Clone)]
struct Point {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
}

struct MainState {
    p: Point,
    point: Vector,
    points: Vec<Vector>,
    mesh: Mesh,
    pathes: Vec<Vec<Point>>,
}

impl Point {
    fn new(x: f32, y: f32, vx: f32, vy: f32) -> Point {
        Point { x, y, vx, vy }
    }

    fn apply_force(&mut self, func: fn(f32, f32) -> f32) -> Result<()> {
        let value = func(self.x, self.y);
        self.vx += (value).cos() * 0.8;
        self.vy += (value).sin() * 0.8;

        self.x += self.vx;
        self.y += self.vy;
        // apply some friction so point doesn't speed up too much
        self.vx *= 0.99;
        self.vy *= 0.99;

        // wrap around edges of screen
        if self.x > WIDTH {
            self.x = 0.
        };
        if self.y > HEIGHT {
            self.y = 0.
        };
        if self.x < 0. {
            self.x = WIDTH
        };
        if self.y < 0. {
            self.y = HEIGHT
        };
        Ok(())
    }
}

impl MainState {
    fn draw_field1(&self, window: &mut Window, func: impl Fn(f32, f32) -> f32) -> Result<()> {
        let step = 20.;
        for x in angle_iter::linspace(0., WIDTH, (WIDTH / step) as u32) {
            for y in angle_iter::linspace(0., HEIGHT, (HEIGHT / step) as u32) {
                let angle = Transform::rotate(func(x, y) * to_degrees());
                let pos = Vector::new(x, y);
                window.draw_ex(
                    &Line::new(pos, pos + angle * Vector::new(step, 0.)),
                    Col(Color::WHITE),
                    Transform::IDENTITY,
                    0.,
                );
            }
        }
        Ok(())
    }

    fn draw_field2(&self, window: &mut Window, func: impl Fn(f32, f32) -> f32) -> Result<()> {
        let mut rng = rand::thread_rng();
        let count = 20000;
        for _ in 0..count {
            let x = rng.gen_range(0., WIDTH);
            let y = rng.gen_range(0., HEIGHT);

            let angle = Transform::rotate(func(x, y) * to_degrees());
            let pos = Vector::new(x, y);
            window.draw_ex(
                &Line::new(pos, pos + angle * Vector::new(20., 0.)),
                Col(Color::WHITE),
                Transform::IDENTITY,
                0.,
            );
        }

        Ok(())
    }

    fn draw_field3(&mut self, window: &mut Window, func: fn(f32, f32) -> f32) -> Result<()> {
        let mut rng = rand::thread_rng();
        let count = 2000;
        for _ in 0..count {
            let x = rng.gen_range(0., WIDTH);
            let y = rng.gen_range(0., HEIGHT);
            let angle = Transform::rotate(func(x, y));
            let pos = Transform::translate((x, y));
            window.draw_ex(
                &Line::new((0., 0.), (30. + rng.gen_range(0., 30.), 1.)),
                Col(Color::BLACK),
                pos * angle,
                0.,
            );
        }

        Ok(())
    }

    fn draw_path(
        window: &mut Window,
        path: &mut Vec<Point>,
        func: fn(f32, f32) -> f32,
    ) -> Result<()> {
        let size = path.len() - 1;
        path[size].apply_force(func)?;
        path.push(Point::new(
            path[size].x,
            path[size].y,
            path[size].vx,
            path[size].vy,
        ));

        for i in 0..path.len() {
            // let angle = Transform::IDENTITY;
            // let pos = Transform::translate((path[i].x, path[i].y));

            // window.draw_ex(
            //     &Line::new((0., 0.), (path[i].vx, path[i].vy)),
            //     Col(Color::BLACK),
            //     pos * angle,
            //     0.,
            // );
            let angle = Transform::IDENTITY;
            let pos = Vector::new(path[i].x, path[i].y);
            window.draw_ex(
                &Line::new(pos, pos + angle * Vector::new(path[i].vx, path[i].vy))
                    .with_thickness(5.),
                Col(Color::WHITE),
                Transform::IDENTITY,
                0.,
            );
        }

        Ok(())
    }
}

impl State for MainState {
    fn new() -> Result<MainState> {
        Ok(MainState {
            points: Vec::new(),
            mesh: Mesh::new(),
            point: Vector::new(0., 0.),
            p: Point::new(0., 0., 0., 0.),
            pathes: vec![
                vec![Point::new(WIDTH / 2. + 10., HEIGHT / 2., 0., 0.)],
                vec![Point::new(WIDTH / 2., HEIGHT / 2. + 10., 0., 0.)],
                vec![Point::new(WIDTH / 2. - 10., HEIGHT / 2., 0., 0.)],
                vec![Point::new(WIDTH / 2., HEIGHT / 2. - 10., 0., 0.)],
            ],
        })
    }

    fn event(&mut self, event: &Event, window: &mut Window) -> Result<()> {
        match *event {
            Event::MouseButton(MouseButton::Left, ButtonState::Pressed) => {}
            Event::Key(Key::Escape, ButtonState::Pressed) => {
                window.close();
            }
            _ => (),
        }
        Ok(())
    }

    fn update(&mut self, _window: &mut Window) -> Result<()> {
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::BLACK)?;
        window.set_fullscreen(true);
        // window.mesh().extend(&self.mesh);
        // self.draw_field1(window, |x, y| (x + y) * 0.001 * std::f32::consts::PI * 2.)?;
        // self.draw_field1(window, clifford_attractor)?;
        // self.draw_field1(window, |x, y| {
        //     ((x - WIDTH / 2.) * (x - WIDTH / 2.) + (y - HEIGHT / 2.) * (y - HEIGHT / 2.)).sqrt()
        //         * 0.001
        //         * std::f32::consts::PI
        //         * 2.
        // })?;
        // self.draw_field2(window, |x, y| (x + y) * 0.001 * std::f32::consts::PI * 2.)?;
        // self.draw_field2(window, clifford_attractor)?;

        // self.draw_field2(window, |x, y| {
        //     ((x - WIDTH / 2.) * (x - WIDTH / 2.) + (y - HEIGHT / 2.) * (y - HEIGHT / 2.)).sqrt()
        //         * 0.001
        //         * std::f32::consts::PI
        //         * 2.
        // })?;
        // self.draw_field1(window, |x, y| {
        //     ((x).sin() + (y).sin()) * 0.05 * std::f32::consts::PI * 2.
        // })?;
        // self.draw_field2(window, |x, y| {
        //     ((x).sin() + (y).sin()) * 0.05 * std::f32::consts::PI * 2.
        // })?;
        // self.draw_field2(window, |x, y| {
        //     ((x * 0.01).sin() + (y * 0.01).sin()) * std::f32::consts::PI * 2.
        // })?;
        // for path in &mut self.pathes {
        //     MainState::draw_path(window, path, |x, y| (x + y))?;
        // }
        // for path in &mut self.pathes {
        //     MainState::draw_path(window, path, |x, y| {
        //         ((x * 0.01).sin() + (y * 0.01).sin()) * std::f32::consts::PI * 2.
        //     })?;
        // }
        // for path in &mut self.pathes {
        //     MainState::draw_path(window, path, |x, y| {
        //         ((x - WIDTH / 2.) * (x - WIDTH / 2.) + (y - HEIGHT / 2.) * (y - HEIGHT / 2.)).sqrt()
        //             * 0.1
        //             * std::f32::consts::PI
        //             * 2.
        //     })?;
        // }
        let step = 5.;
        for x in angle_iter::linspace(0., WIDTH, (WIDTH / step) as u32) {
            for y in angle_iter::linspace(0., HEIGHT, (HEIGHT / step) as u32) {
                let a = 1.6;
                let b = -0.6;
                let c = -1.2;
                let d = 1.6;
                // scale down x and y
                let scale = 80.;
                // attactor gives new x, y for old one.
                let x1 = (a * y).sin() + c * (a * x).cos();
                let y1 = (b * x).sin() + d * (b * y).cos();
                window.draw(
                    &Circle::new((x1 * scale + WIDTH / 2., y1 * scale + HEIGHT / 2.), 0.5),
                    Col(Color::WHITE),
                );
            }
        }

        Ok(())
    }
}

fn main() {
    run::<MainState>(
        "Flow field",
        Vector::new(WIDTH, HEIGHT),
        Settings::default(),
    );
}

// clifford attractor
// http://paulbourke.net/fractals/clifford/
fn clifford_attractor(x: f32, y: f32) -> f32 {
    let a = 1.6;
    let b = -0.6;
    let c = -1.2;
    let d = 1.6;
    // scale down x and y
    let scale = 10.;
    let x = (x) * scale - WIDTH / 2.;
    let y = (y) * scale - HEIGHT / 2.;

    // attactor gives new x, y for old one.
    let x1 = (a * y).sin() + c * (a * x).cos();
    let y1 = (b * x).sin() + d * (b * y).cos();

    // find angle from old to new. that's the value.
    (y1 - y).atan2(x1 - x)
}

fn to_radian() -> f32 {
    std::f32::consts::PI / 180.
}

fn to_degrees() -> f32 {
    180. / std::f32::consts::PI
}
