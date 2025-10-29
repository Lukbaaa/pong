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
use std::f64::consts::PI;

const WIDTH: f64 = 800f64;
const HEIGHT: f64 = 800f64;

pub struct App {
    gl: GlGraphics,
    pressed_keys: HashSet<Key>,
    player1: Player,
    player2: Player,
    ball: Ball,
    is_started: bool,
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

impl Player {
    fn collided(&self, ball: &Ball) -> bool {
        let p_lx = self.position.x;
        let p_rx = self.position.x + self.size * self.ratio;
        let p_uy = self.position.y;
        let p_dy = self.position.y + self.size;
        let b_lx = ball.position.x - ball.size;
        let b_rx = ball.position.x + ball.size;
        let _b_uy = ball.position.y - ball.size;
        let _b_dy = ball.position.y + ball.size;
        let b_y = ball.position.y;

        (p_uy < b_y && b_y < p_dy) && (b_lx < p_rx && b_rx > p_lx)
    }

    fn collision_point(&self, ball: &Ball) -> f64 {
        let b_dy = ball.position.y - self.position.y;
        let b_ry = b_dy / self.size;

        b_ry * 2.0 - 1.0
    }
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
        let player_speed = 5.0;
        let ball_speed = 3.0;

        for key in &self.pressed_keys {
            match key {
                Key::W => self.player1.position.y -= player_speed,
                Key::S => self.player1.position.y += player_speed,
                Key::Up => self.player2.position.y -= player_speed,
                Key::Down => self.player2.position.y += player_speed,
                _ => {}
            }
            if !self.is_started {
                match key {
                    Key::W => self.is_started = true,
                    Key::S => self.is_started = true,
                    _ => {}
                }
            }
        }

        if self.player1.position.y < 0.0 {
            self.player1.position.y += player_speed;
        }
        if self.player1.position.y > 800.0 - self.player1.size {
            self.player1.position.y -= player_speed;
        }
        if self.player2.position.y < 0.0 {
            self.player2.position.y += player_speed;
        }
        if self.player2.position.y > 800.0 - self.player1.size {
            self.player2.position.y -= player_speed;
        }

        if self.is_started {
            self.ball.position.x += self.ball.angle.cos() * ball_speed;
            self.ball.position.y -= self.ball.angle.sin() * ball_speed;
        }

        if self.ball.position.y - self.ball.size < 0.0
            || self.ball.position.y + self.ball.size > HEIGHT
        {
            self.ball.angle = -self.ball.angle;
        }

        if self.player1.collided(&self.ball) {
            self.ball.angle = -(75.0 * PI / 180.0) * self.player1.collision_point(&self.ball);
        }

        if self.player2.collided(&self.ball) {
            self.ball.angle = PI + (75.0 * PI / 180.0) * self.player2.collision_point(&self.ball);
        }

        // TODO Game logic
        // - Collision with player
        // - Goal
        // - Winning
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

    let mut window: Window = WindowSettings::new("Pong", [WIDTH, HEIGHT])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let player1 = Player {
        size: 80.0,
        ratio: 0.2,
        position: Position {
            x: 50.0,
            y: HEIGHT / 2.0,
        },
    };

    let player2 = Player {
        size: 80.0,
        ratio: 0.2,
        position: Position {
            x: WIDTH - 50.0,
            y: HEIGHT / 2.0,
        },
    };

    let ball = Ball {
        size: 10.0,
        position: Position {
            x: player1.position.x + player1.size * player1.ratio + 20.0,
            y: player1.position.y + player1.size / 2.0,
        },
        angle: 0.0, //radians
    };

    let mut app = App {
        gl: GlGraphics::new(opengl),
        pressed_keys: HashSet::new(),
        player1,
        player2,
        ball,
        is_started: false,
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
