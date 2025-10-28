extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::Button;
use piston::Key;
use piston::PressEvent;
use piston::ReleaseEvent;
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use std::collections::HashSet;

pub struct App {
    gl: GlGraphics,
    pressed_keys: HashSet<Key>,
    player1: Player,
    player2: Player,
    ball: Ball,
}

pub struct Position {
    x: f64,
    y: f64,
}

pub struct Player {
    size: f64,
    ratio: f64,
    position: Position,
}

pub struct Ball {
    size: f64,
    position: Position,
    angle: f64,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        let ball = ellipse::circle(0.0, 0.0, self.ball.size);

        self.gl.draw(args.viewport(), |c, gl| {
            clear(WHITE, gl);

            let ball_transform = c
                .transform
                .trans(self.ball.position.x, self.ball.position.y);

            let player1_transform = c
                .transform
                .trans(self.player1.position.x, self.player1.position.y);

            let player2_transform = c
                .transform
                .trans(self.player2.position.x, self.player2.position.y);

            let from = [0.0, 0.0];
            let to = [self.player1.size * self.player1.ratio, self.player1.size];

            ellipse(BLACK, ball, ball_transform, gl);
            rectangle_from_to(BLACK, from, to, player1_transform, gl);
            rectangle_from_to(BLACK, from, to, player2_transform, gl);
        });
    }

    fn update(&mut self, _args: &UpdateArgs) {
        let speed = 5.0;

        for key in &self.pressed_keys {
            match key {
                Key::W => self.player1.position.y -= speed,
                Key::S => self.player1.position.y += speed,
                Key::Up => self.player2.position.y -= speed,
                Key::Down => self.player2.position.y += speed,
                _ => {}
            }
        }

        if self.player1.position.y < 0.0 {
            self.player1.position.y += speed;
        }
        if self.player1.position.y > 800.0 - self.player1.size {
            self.player1.position.y -= speed;
        }
        if self.player2.position.y < 0.0 {
            self.player2.position.y += speed;
        }
        if self.player2.position.y > 800.0 - self.player1.size {
            self.player2.position.y -= speed;
        }

        // TODO Ball movement

        // TODO Game logic
    }

    fn key_press(&mut self, key: Key) {
        self.pressed_keys.insert(key);
    }

    fn key_release(&mut self, key: Key) {
        self.pressed_keys.remove(&key);
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("Pong", [800, 800])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let player1 = Player {
        size: 80.0,
        ratio: 0.2,
        position: Position { x: 50.0, y: 400.0 },
    };

    let player2 = Player {
        size: 80.0,
        ratio: 0.2,
        position: Position { x: 750.0, y: 400.0 },
    };

    let ball = Ball {
        size: 20.0,
        position: Position { x: 500.0, y: 400.0 },
        angle: 0.0,
    };

    let mut app = App {
        gl: GlGraphics::new(opengl),
        pressed_keys: HashSet::new(),
        player1,
        player2,
        ball,
    };

    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            app.key_press(key);
        }

        if let Some(Button::Keyboard(key)) = e.release_args() {
            app.key_release(key);
        }
    }
}
