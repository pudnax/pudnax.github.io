// Draw the classic triangle to the screen
extern crate noise;
extern crate quicksilver;
extern crate rand;

mod noise_loop;
use noise_loop::map;

use quicksilver::{
    combinators::result,
    geom::{Line, Shape, Vector},
    graphics::{Background::Col, Background::Img, Color, Font, FontStyle, Image},
    input::{ButtonState, Key, MouseButton},
    lifecycle::{run, Asset, Event, Settings, State, Window},
    Future, Result,
};

const WIDTH: f32 = 800.;
const HEIGHT: f32 = 500.;

struct MainState {
    asset: Asset<Image>,
    res: f64,
    pi: PiLeibniz,
    history: Vec<f32>,
    counter: f32,
}

impl MainState {
    fn draw_lines(&mut self, window: &mut Window) -> Result<()> {
        let spacing = WIDTH / self.history.len() as f32;

        let points: Vec<(f32, f32)> = (0..self.history.len())
            // .map(|x| (x as f32, map(self.history[x], 0., 3.5, HEIGHT, 0.)))
            .map(|x| {
                (
                    x as f32 * spacing,
                    map(self.history[x], 2., 3.5, -HEIGHT / 2., HEIGHT / 2.) + HEIGHT / 4.,
                )
            })
            .collect();
        for i in 0..points.len() - 1 {
            window.draw(&Line::new(points[i], points[i + 1]), Col(Color::WHITE))
        }
        Ok(())
    }
}

impl State for MainState {
    fn new() -> Result<MainState> {
        let asset = Asset::new(Font::load("font.ttf").and_then(|font| {
            let style = FontStyle::new(72.0, Color::BLACK);
            result(font.render("Sample Text", &style))
        }));

        Ok(MainState {
            asset,
            pi: PiLeibniz::new(),
            res: 0.,
            history: vec![HEIGHT / 2.],
            counter: 0.,
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
        self.res += self.pi.next().unwrap();
        let ass = |text: f64| {
            Asset::new(Font::load("font.ttf").and_then(move |font| {
                let style = FontStyle::new(48.0, Color::WHITE);
                result(font.render(&format!("{:.04}", text), &style))
            }))
        };
        if self.counter as i32 % 100 == 0 {
            self.asset = ass(self.res);
        }
        // self.text = graphics::Text::new((format!("{}", self.res), self.font, 32.0));
        self.history.push(self.res as f32);
        self.counter += 1.;
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::BLACK)?;
        // window.set_view(View::new(Rectangle::new(
        //     (0, -HEIGHT / 2.),
        //     (WIDTH, HEIGHT),
        // )));
        self.asset.execute(|image| {
            window.draw(&image.area().with_center((WIDTH - 120., 50.)), Img(&image));
            Ok(())
        })?;
        self.draw_lines(window)?;

        Ok(())
    }
}

fn main() {
    run::<MainState>(
        "Leibniz formula",
        Vector::new(WIDTH, HEIGHT),
        Settings::default(),
    );
}

struct PiLeibniz {
    presize: f64,
}

impl PiLeibniz {
    fn new() -> PiLeibniz {
        PiLeibniz { presize: 0. }
    }
}

impl Iterator for PiLeibniz {
    type Item = f64;

    fn next(&mut self) -> Option<f64> {
        let den = 2. * self.presize + 1.;
        self.presize += 1.;
        Some(4. * (-1f64).powf(self.presize - 1.) * (1. / den))
    }
}
