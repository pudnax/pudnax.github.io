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
    m: usize,
    index: usize,
    value: f32,
    pivot_index: usize,
    pivot: Vector,
    walking_index: usize,
    points: Vec<Vector>,
    mesh: Mesh,
    sorting: bool,
    wraping: bool,
}

const POINTS_NUM: usize = 30;

fn ccw(a: Vector, b: Vector, c: Vector) -> f32 {
    ((b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x))
}

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

        let mut low_index = 0;
        for i in 1..points.len() {
            if points[i].y < points[low_index].y {
                low_index = i;
            }
        }
        points.swap(0, low_index);
        let pivot = points[0];

        self.m = 1;
        self.index = 2;
        self.value = std::f32::MAX;
        self.pivot_index = 0;
        self.walking_index = 0;
        self.pivot = pivot;
        self.points = points;
        self.mesh = Mesh::new();
        self.sorting = true;
        self.wraping = true;
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

        let mut low_index = 0;
        for i in 1..points.len() {
            if points[i].y < points[low_index].y {
                low_index = i;
            }
        }
        points.swap(0, low_index);
        let pivot = points[0];

        // points.sort_by(|&a, &b| ccw(pivot, a, b).partial_cmp(&0.).unwrap());
        // points.sort_by(|&a, &b| ((a - pivot).cross(b - pivot)).partial_cmp(&0.).unwrap());

        // let mut val = std::f32::MAX;
        // for i in 0..points.len() {
        //     for j in 0..points.len() {
        //         if (points[i] - pivot).cross(points[j] - pivot) < val {
        //             points.swap(i, j);
        //             val = (points[i] - pivot).cross(points[j] - pivot);
        //         }
        //     }
        //     // val = std::f32::MAX;
        // }

        Ok(MainState {
            m: 1,
            index: 2,
            value: std::f32::MAX,
            pivot_index: 0,
            walking_index: 0,
            pivot,
            points,
            mesh: Mesh::new(),
            sorting: true,
            wraping: true,
        })
    }

    fn event(&mut self, event: &Event, window: &mut Window) -> Result<()> {
        match *event {
            Event::MouseButton(MouseButton::Left, ButtonState::Pressed) => {
                self.refresh()?;
            }
            Event::Key(Key::Escape, ButtonState::Pressed) => {
                window.close();
            }
            _ => (),
        }
        Ok(())
    }

    fn update(&mut self, _window: &mut Window) -> Result<()> {
        if self.sorting {
            if (self.points[self.walking_index] - self.pivot)
                .cross(self.points[self.pivot_index] - self.pivot)
                < self.value
            {
                self.points.swap(self.walking_index, self.pivot_index);
                self.value = (self.points[self.walking_index] - self.pivot)
                    .cross(self.points[self.pivot_index] - self.pivot);
            }

            self.walking_index += 1;

            if self.walking_index == self.points.len() {
                self.pivot_index += 1;
                self.walking_index = 0;
            }
            if self.pivot_index == self.points.len() {
                self.sorting = false;
            }
        } else {
            if self.wraping {
                while ccw(
                    self.points[self.m - 1],
                    self.points[self.m],
                    self.points[self.index],
                ) <= 0.
                {
                    if self.m > 1 {
                        self.m -= 1;
                        continue;
                    } else if self.index == self.points.len() {
                        break;
                    } else {
                        self.index += 1;
                    }
                }
                self.m += 1;
                self.points.swap(self.index, self.m);
                self.index += 1;
                if self.index == self.points.len() {
                    self.wraping = false;
                }
            }
            self.mesh.vertices = self.points[0..=self.m]
                .iter()
                .map(|x| Vertex::new((x.x, x.y), None, Col(Color::from_rgba(0, 0, 200, 0.2))))
                .collect::<Vec<_>>();
            let triangle_count = self.mesh.vertices.len() as u32 - 1;
            for index in 0..triangle_count {
                self.mesh.triangles.push(GpuTriangle {
                    z: 0.0,
                    indices: [0, index as u32 + 1, (index as u32 + 1) % triangle_count + 1],
                    image: None,
                });
            }
        }

        // if !self.wraping {
        //     self.refresh()?;
        // }
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::BLACK)?;
        window.mesh().extend(&self.mesh);

        if self.points.len() > 2 {
            for i in 0..self.points.len() - 1 {
                window.draw(
                    &Line::new(self.points[i], self.points[i + 1]),
                    Col(Color::RED),
                );
            }
        }
        for point in &self.points {
            self.draw_circle(*point, Color::WHITE, window)?;
        }

        window.draw(
            &Line::new(
                self.points[self.pivot_index % self.points.len()],
                self.points[self.walking_index],
            ),
            Col(Color::BLUE),
        );

        Ok(())
    }
}

fn main() {
    run::<MainState>(
        "Braham scan",
        Vector::new(WIDTH, HEIGHT),
        Settings::default(),
    );
}
