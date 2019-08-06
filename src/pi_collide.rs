// Draw the classic triangle to the screen
extern crate noise;
extern crate quicksilver;
extern crate rand;

mod noise_loop;

use quicksilver::{
    combinators::result,
    geom::{Rectangle, Shape, Vector},
    graphics::{Background::Col, Background::Img, Color, Font, FontStyle, Image},
    input::{ButtonState, Key, MouseButton},
    lifecycle::{run, Asset, Event, Settings, State, Window},
    sound::Sound,
    Future, Result,
};

const WIDTH: f32 = 800.;
const HEIGHT: f32 = 300.;
const DIGITS: f32 = 1.;

struct MainState {
    pos_pusher: Box,
    pos_collider: Box,
    count: f64,
    connect: bool,
    sound: Asset<Sound>,
    face: Asset<Image>,
    text: Asset<Image>,
    time_steps: f32,
    wall: Rectangle,
    ground: Rectangle,
}

impl State for MainState {
    fn new() -> Result<MainState> {
        let text = Asset::new(Font::load("font.ttf").and_then(|font| {
            let style = FontStyle::new(72.0, Color::BLACK);
            result(font.render("Sample Text", &style))
        }));
        let sound = Asset::new(Sound::load("clack.wav"));
        let face = Asset::new(Image::load("pi_face.png"));

        let time_steps = (10f32).powf(DIGITS - 1.);
        let m = (100f32).powf(DIGITS - 1.);

        let ground = Rectangle::new((0., HEIGHT / 2.), (WIDTH, HEIGHT));
        let wall = Rectangle::new((0., 0.), (50., HEIGHT / 2.));

        Ok(MainState {
            pos_collider: Box::new(WIDTH / 2. - 250., HEIGHT / 2., 20., 1., 0.)?,
            pos_pusher: Box::new(WIDTH / 2., HEIGHT / 2., 40., m, -0.5 / time_steps as f32)?,
            time_steps,

            count: 0.,
            connect: false,
            wall,
            ground,
            sound,
            face,
            text,
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

    fn update(&mut self, window: &mut Window) -> Result<()> {
        window.set_max_updates(1);
        let ass = |text: f64| {
            Asset::new(Font::load("font.ttf").and_then(move |font| {
                let style = FontStyle::new(48.0, Color::WHITE);
                result(font.render(&format!("{}", text), &style))
            }))
        };

        for _i in 0..self.time_steps as isize {
            if self.pos_pusher.collide(&self.pos_collider) {
                let v1 = self.pos_pusher.bounce(&self.pos_collider);
                let v2 = self.pos_collider.bounce(&self.pos_pusher);
                self.pos_pusher.vel = v1;
                self.pos_collider.vel = v2;
                self.count += 1.;
                self.connect = true;
            }
            if self.pos_pusher.hit_wall() {
                self.pos_pusher.reverse();
                self.count += 1.;
                self.connect = true;
            }
            if self.pos_collider.hit_wall() {
                self.pos_collider.reverse();
                self.count += 1.;
                self.connect = true;
            }
            if self.connect {
                self.sound.execute(|sound| {
                    sound.play()?;
                    Ok(())
                })?;
                self.text = ass(self.count);
            }
            self.connect = false;
            self.pos_collider.hit_wall();
            self.pos_pusher.update();
            self.pos_collider.update();
        }

        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::BLACK)?;

        self.pos_pusher.draw(window, &mut self.face)?;
        self.pos_collider.draw(window, &mut self.face)?;

        window.draw(&self.wall, Col(Color::from_rgba(64, 64, 64, 1.)));
        window.draw(&self.ground, Col(Color::from_rgba(88, 88, 88, 1.)));

        self.text.execute(|image| {
            window.draw(&image.area().with_center((WIDTH - 120., 50.)), Img(&image));
            Ok(())
        })?;

        Ok(())
    }
}

fn main() {
    run::<MainState>(
        "Pi collide",
        Vector::new(WIDTH, HEIGHT),
        Settings::default(),
    );
}

struct Box {
    w: f32,
    x: f32,
    y: f32,
    mass: f32,
    vel: f32,
}

impl Box {
    fn new(x: f32, y: f32, w: f32, mass: f32, vel: f32) -> Result<Box> {
        let b = Box {
            w,
            x: x + w,
            y: y - w / 2.,
            mass,
            vel,
        };
        Ok(b)
    }

    fn draw(&mut self, window: &mut Window, face: &mut Asset<Image>) -> Result<()> {
        let x = self.x - self.w / 2.;
        let y = self.y - self.w / 2.;
        let w = self.w;

        face.execute(|image| {
            window.draw(&Rectangle::new((x, y), (w, w)), Img(&image));
            Ok(())
        })?;

        Ok(())
    }

    fn hit_wall(&self) -> bool {
        self.x - self.w / 2. <= 50.
    }

    fn reverse(&mut self) {
        self.vel *= -1.;
    }

    fn bounce(&self, other: &Box) -> f32 {
        let summ = self.mass + other.mass;
        self.vel * (self.mass - other.mass) / summ + other.vel * 2. * other.mass / summ
    }

    fn collide(&self, other: &Box) -> bool {
        !(self.x - self.w / 2. > other.x + other.w / 2.
            || self.x + self.w / 2. < other.x - other.w / 2.)
    }

    fn update(&mut self) {
        self.x += self.vel;
    }
}
