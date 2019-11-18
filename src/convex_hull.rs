// Draw the classic triangle to the screen
extern crate quicksilver;
extern crate rand;

use rand::{thread_rng, Rng};

mod angle_iter;

use quicksilver::{
    geom::{Circle, Line, Vector},
    graphics::{Background::Col, Color, GpuTriangle, Mesh, Vertex},
    input::{ButtonState, Key, MouseButton},
    lifecycle::{run, Event, Settings, State, Window},
    Result,
};

const WIDTH: f32 = 1100.;
const HEIGHT: f32 = 800.;

struct MainState {
    index: usize,
    next_index: i32,
    left_most: Vector,
    current_vertex: Vector,
    next_vertex: Vector,
    hull: Vec<Vector>,
    points: Vec<Vector>,
    mesh: Mesh,
}

const POINTS_NUM: usize = 50;

impl MainState {
    fn draw_circle(&self, point: Vector, col: Color, window: &mut Window) -> Result<()> {
        window.draw(&Circle::new((point.x, point.y), 5.), Col(col));
        Ok(())
    }

    fn draw_line(&mut self, at: Vector, to: Vector, col: Color, window: &mut Window) -> Result<()> {
        if at != to {}
        window.draw(&Line::new((at.x, at.y), (to.x, to.y)), Col(col));
        Ok(())
    }

    fn refresh(&mut self) -> Result<()> {
        let mut rng = thread_rng();
        let points_num = POINTS_NUM;
        let buffer = 40.;

        let mut points: Vec<Vector> = Vec::with_capacity(points_num);
        for _ in 0..points_num {
            points.push(Vector::new(
                rng.gen_range(buffer, WIDTH - buffer),
                rng.gen_range(buffer, HEIGHT - buffer),
            ));
        }
        points.sort_by(|a, b| (a.x - b.x).partial_cmp(&0.).unwrap());

        let mut hull: Vec<Vector> = Vec::new();
        hull.push(points[0]);

        let mut mesh = Mesh::new();
        mesh.vertices.push(Vertex {
            pos: points[0],
            tex_pos: None,
            col: Color::RED,
        });

        self.index = 2;
        self.next_index = -1;
        self.left_most = points[0];
        self.current_vertex = points[0];
        self.next_vertex = points[1];
        self.points = points;
        self.hull = hull;
        self.mesh = mesh;

        Ok(())
    }
}

impl State for MainState {
    fn new() -> Result<MainState> {
        let mut rng = thread_rng();
        let points_num = POINTS_NUM;
        let buffer = 40.;

        let mut points: Vec<Vector> = Vec::with_capacity(points_num);
        for _ in 0..points_num {
            points.push(Vector::new(
                rng.gen_range(buffer, WIDTH - buffer),
                rng.gen_range(buffer, HEIGHT - buffer),
            ));
        }
        points.sort_by(|a, b| (a.x - b.x).partial_cmp(&0.).unwrap());

        let mut hull: Vec<Vector> = Vec::new();
        hull.push(points[0]);

        let mut mesh = Mesh::new();
        mesh.vertices.push(Vertex {
            pos: points[0],
            tex_pos: None,
            col: Color::RED,
        });
        Ok(MainState {
            index: 2,
            next_index: -1,
            left_most: points[0],
            current_vertex: points[0],
            next_vertex: points[1],
            points,
            hull,
            mesh,
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
        // self.mesh.clear();
        let a = self.next_vertex - self.current_vertex;
        let b = self.points[self.index] - self.current_vertex;
        let cross = a.cross(b);

        if cross < 0. {
            self.next_vertex = self.points[self.index];
            self.next_index = self.index as i32;
        }

        self.index += 1;
        if self.index == self.points.len() {
            if self.next_vertex == self.left_most {
                self.refresh()?;
            } else {
                self.hull.push(self.next_vertex);
                self.mesh.vertices.push(Vertex {
                    pos: self.next_vertex,
                    tex_pos: None,
                    col: Color::RED,
                });
                self.current_vertex = self.next_vertex;
                self.index = 0;
                self.next_vertex = self.left_most;
            }
        }

        let triangle_count = self.mesh.vertices.len() as u32 - 1;
        for index in 0..triangle_count {
            self.mesh.triangles.push(GpuTriangle {
                z: 0.0,
                indices: [0, index as u32 + 1, (index as u32 + 1) % triangle_count + 1],
                image: None,
            });
        }
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::BLACK)?;
        window.mesh().extend(&self.mesh);

        self.draw_circle(self.left_most, Color::from_rgba(255, 0, 0, 1.), window)?;
        self.draw_circle(
            self.current_vertex,
            Color::from_rgba(255, 255, 0, 1.),
            window,
        )?;
        self.draw_line(
            self.current_vertex,
            self.left_most,
            Color::from_rgba(0, 0, 255, 0.7),
            window,
        )?;

        self.draw_line(
            self.current_vertex,
            self.next_vertex,
            Color::from_rgba(0, 255, 255, 0.7),
            window,
        )?;

        self.draw_line(
            self.current_vertex,
            self.points[self.index],
            Color::from_rgba(255, 0, 255, 0.7),
            window,
        )?;
        for point in &self.points {
            self.draw_circle(*point, Color::WHITE, window)?;
        }

        Ok(())
    }
}

fn main() {
    run::<MainState>(
        "Convex_hull",
        Vector::new(WIDTH, HEIGHT),
        Settings::default(),
    );
}
