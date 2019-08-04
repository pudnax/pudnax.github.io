extern crate noise;
extern crate quicksilver;
extern crate rand;

mod angle_iter;

mod structures;
use structures::{boundary::Boundary, rays::Ray, vect2d::Vect2D};

use quicksilver::{
    geom::{Circle, Line, Vector},
    graphics::{Background::Col, Color, Vertex},
    lifecycle::{run, Event, Settings, State, Window},
    Result,
};

const WIDTH: f32 = 1100.;
const HEIGHT: f32 = 800.;

struct MainState {
    total_frames: u32,
    particle: Particle,
    walls: Vec<Boundary>,
    lines: Vec<Vect2D>,
    counter: u32,
    record: bool,
    vertices: Vec<Vertex>,
}

impl MainState {
    fn draw_line(&mut self, window: &mut Window, line: Vect2D) -> Result<()> {
        let metric = (self.particle.pos - line).len();
        let max = (WIDTH * WIDTH + HEIGHT * HEIGHT).sqrt();

        window.draw(
            &Line::new((self.particle.pos.x, self.particle.pos.y), (line.x, line.y))
                .with_thickness(0.5 + 10. * metric / max),
            Col(Color::from_rgba(255, 255, 255, 1. - 2. * metric / max)),
        );
        Ok(())
    }

    fn update_intersect(&mut self) {
        self.lines.clear();
        let posx = self.particle.pos.x;
        let posy = self.particle.pos.y;

        //        for ray in &mut self.particle.rays {
        //            for wall in &mut self.walls {
        //                match ray.cast(*wall) {
        //                    Some(x) => self.lines.push(x),
        //                    None => {}
        //                }
        //            }
        for ray in &mut self.particle.rays {
            let mut buff = Vec::new();
            for wall in &mut self.walls {
                if let Some(x) = ray.cast(*wall) {
                    buff.push(x)
                }
            }

            if let Some(x) = buff.iter().min_by(move |&a, &b| {
                let quad = |point: Vect2D| {
                    (posx - point.x) * (posx - point.x) + (posy - point.y) * (posy - point.y)
                };
                (&quad(*a)).partial_cmp(&quad(*b)).unwrap()
            }) {
                self.lines.push(*x)
            }
        }
    }
}

impl State for MainState {
    fn new() -> Result<MainState> {
        let mut walls = Vec::new();
        walls.push(Boundary::new(0., 0., WIDTH, 0.));
        walls.push(Boundary::new(0., 0., 0., HEIGHT));
        walls.push(Boundary::new(WIDTH, 0., WIDTH, HEIGHT));
        walls.push(Boundary::new(0., HEIGHT, WIDTH, HEIGHT));
        Ok(MainState {
            record: false,
            total_frames: 600,
            counter: 0,
            particle: Particle::new(WIDTH / 2., HEIGHT / 2.),
            lines: Vec::new(),
            walls,
            vertices: Vec::new(),
        })
    }

    // fn event(&mut self, event: &Event, _window: &mut Window) -> Result<()> {
    //     if let Event::MouseButton(_, state) = *event {
    //         if state == quicksilver::input::ButtonState::Pressed {
    //             self.walls = (0..5)
    //                 .map(|_x| Boundary::random(WIDTH, HEIGHT))
    //                 .collect::<Vec<Boundary>>();
    //             self.walls.push(Boundary::new(0., 0., WIDTH, 0.));
    //             self.walls.push(Boundary::new(0., 0., 0., HEIGHT));
    //             self.walls.push(Boundary::new(WIDTH, 0., WIDTH, HEIGHT));
    //             self.walls.push(Boundary::new(0., HEIGHT, WIDTH, HEIGHT));
    //         }
    //     }
    //     Ok(())
    // }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        window.set_max_updates(1);
        self.vertices = Vec::new();

        if window.mouse()[quicksilver::input::MouseButton::Left]
            == quicksilver::input::ButtonState::Pressed
        {
            self.walls = (0..5)
                .map(|_x| Boundary::random(WIDTH, HEIGHT))
                .collect::<Vec<Boundary>>();
            self.walls.push(Boundary::new(0., 0., WIDTH, 0.));
            self.walls.push(Boundary::new(0., 0., 0., HEIGHT));
            self.walls.push(Boundary::new(WIDTH, 0., WIDTH, HEIGHT));
            self.walls.push(Boundary::new(0., HEIGHT, WIDTH, HEIGHT));
        }

        let mouse_pos = window.mouse().pos();
        self.particle = Particle::new(mouse_pos.x, mouse_pos.y);
        self.update_intersect();

        if self.record {
            let im = window.screenshot(quicksilver::graphics::PixelFormat::RGBA);
            std::fs::create_dir_all("./for_gif")?;
            im.save(&format!("./for_gif/frame{:04}.png", self.counter))?;
            if self.counter == self.total_frames {
                self.record = false;
                println!("record done!")
            }
        }
        self.counter += 1;
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::BLACK)?;

        self.particle.draw(window)?;
        for i in 0..self.lines.len() {
            self.draw_line(window, self.lines[i])?;
        }

        for wall in &mut self.walls {
            wall.draw(window)?;
        }

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

struct Particle {
    pos: Vect2D,
    rays: Vec<Ray>,
}

impl Particle {
    fn new(x: f32, y: f32) -> Particle {
        let mut rays = Vec::new();

        for i in angle_iter::Angle::new(0.01) {
            rays.push(Ray::new(x, y, i.cos() as f32, i.sin() as f32))
        }
        Particle {
            pos: Vect2D::new(x, y),
            rays,
        }
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.draw(
            &Circle::new((self.pos.x, self.pos.y), 5.),
            Col(Color::WHITE),
        );
        Ok(())
    }
}
