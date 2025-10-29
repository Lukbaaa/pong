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
use piston::window::Window as WindowTrait;
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
    kick_off: usize,
    ball: Ball,
    is_started: bool,
    game_over: bool,
    score: [u32; 2],
    winner: usize,
}

pub struct Position {
    x: f64,
    y: f64,
}

pub struct Player {
    height: f64,
    width: f64,
    speed: f64,
    position: Position,
}

pub struct Ball {
    radius: f64,
    speed: f64,
    angle: f64,
    position: Position,
}

impl Player {
    fn collided(&self, ball: &Ball) -> bool {
        let p_lx = self.position.x;
        let p_rx = self.position.x + self.width;
        let p_uy = self.position.y;
        let p_dy = self.position.y + self.height;
        let b_lx = ball.position.x - ball.radius;
        let b_rx = ball.position.x + ball.radius;
        let _b_uy = ball.position.y - ball.radius;
        let _b_dy = ball.position.y + ball.radius;
        let b_y = ball.position.y;

        (p_uy < b_y && b_y < p_dy) && (b_lx < p_rx && b_rx > p_lx)
    }

    fn collision_point(&self, ball: &Ball) -> f64 {
        let b_dy = ball.position.y - self.position.y;
        let b_ry = b_dy / self.height;

        b_ry * 2.0 - 1.0
    }
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        let ball = ellipse::circle(0.0, 0.0, self.ball.radius);

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
            let to = [self.player1.width, self.player1.height];

            ellipse(BLACK, ball, ball_transform, gl);
            rectangle_from_to(BLACK, from, to, player1_transform, gl);
            rectangle_from_to(BLACK, from, to, player2_transform, gl);

            //TODO
            //- render score
            //- render winning screen
        });
    }

    fn update(&mut self, _args: &UpdateArgs) {
        for key in &self.pressed_keys {
            match key {
                Key::W => self.player1.position.y -= self.player1.speed,
                Key::S => self.player1.position.y += self.player1.speed,
                Key::Up => self.player2.position.y -= self.player2.speed,
                Key::Down => self.player2.position.y += self.player2.speed,
                _ => {}
            }
            if !self.is_started {
                match key {
                    Key::W => {
                        if self.kick_off == 1 {
                            self.is_started = true
                        }
                    }
                    Key::S => {
                        if self.kick_off == 1 {
                            self.is_started = true
                        }
                    }
                    Key::Up => {
                        if self.kick_off == 2 {
                            self.is_started = true
                        }
                    }
                    Key::Down => {
                        if self.kick_off == 2 {
                            self.is_started = true
                        }
                    }
                    _ => {}
                }
            }
        }

        if self.player1.position.y < 0.0 {
            self.player1.position.y += self.player1.speed;
        }
        if self.player1.position.y > 800.0 - self.player1.height {
            self.player1.position.y -= self.player1.speed;
        }
        if self.player2.position.y < 0.0 {
            self.player2.position.y += self.player2.speed;
        }
        if self.player2.position.y > 800.0 - self.player1.height {
            self.player2.position.y -= self.player2.speed;
        }

        if self.is_started {
            self.ball.position.x += self.ball.angle.cos() * self.ball.speed;
            self.ball.position.y -= self.ball.angle.sin() * self.ball.speed;
        }

        if self.ball.position.y - self.ball.radius < 0.0
            || self.ball.position.y + self.ball.radius > HEIGHT
        {
            self.ball.angle = -self.ball.angle;
        }

        if self.player1.collided(&self.ball) {
            let collision_point = self.player1.collision_point(&self.ball);
            self.ball.angle = -(75.0 * PI / 180.0) * collision_point;
            self.ball.speed = 3.0 + collision_point.abs() * 5.0;
        }

        if self.player2.collided(&self.ball) {
            let collision_point = self.player2.collision_point(&self.ball);
            self.ball.angle = PI + (75.0 * PI / 180.0) * collision_point;
            self.ball.speed = 3.0 + collision_point.abs() * 3.0;
        }

        if self.ball.position.x < self.player1.position.x {
            self.score[1] += 1;
            self.is_started = false;
            self.kick_off = 1;

            self.ball.angle = 0.0;

            self.ball.position.x = self.player1.position.x + self.player1.width + 20.0;
            self.ball.position.y = HEIGHT / 2.0;

            self.player1.position.y = HEIGHT / 2.0 - 40.0;
            self.player2.position.y = HEIGHT / 2.0 - 40.0;
        }

        if self.ball.position.x > self.player2.position.x {
            self.score[0] += 1;
            self.is_started = false;

            self.kick_off = 2;

            self.ball.angle = PI;

            self.ball.position.x =
                self.player2.position.x + self.player2.width - self.player2.width - 20.0;
            self.ball.position.y = HEIGHT / 2.0;

            self.player1.position.y = HEIGHT / 2.0 - 40.0;
            self.player2.position.y = HEIGHT / 2.0 - 40.0;
        }

        if self.score[0] == 10 {
            self.winner = 1;
            self.game_over = true;
        } else if self.score[1] == 10 {
            self.winner = 2;
            self.game_over = true;
        }
    }

    fn key_press(&mut self, key: Key) {
        self.pressed_keys.insert(key);
    }

    fn key_release(&mut self, key: Key) {
        self.pressed_keys.remove(&key);
    }

    fn current_player(&self) -> &Player {
        match self.kick_off {
            1 => &self.player1,
            2 => &self.player2,
            _ => unreachable!(),
        }
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
        height: 80.0,
        width: 16.0,
        speed: 5.0,
        position: Position {
            x: 50.0,
            y: HEIGHT / 2.0 - 40.0,
        },
    };

    let player2 = Player {
        height: 80.0,
        width: 16.0,
        speed: 5.0,
        position: Position {
            x: WIDTH - 50.0 - 16.0,
            y: HEIGHT / 2.0 - 40.0,
        },
    };

    let ball = Ball {
        radius: 10.0,
        speed: 3.0,
        position: Position {
            x: player1.position.x + player1.width + 20.0,
            y: HEIGHT / 2.0,
        },
        angle: 0.0, //radians
    };

    let mut app = App {
        gl: GlGraphics::new(opengl),
        pressed_keys: HashSet::new(),
        player1,
        player2,
        kick_off: 1,
        ball,
        is_started: false,
        game_over: false,
        score: [0, 0],
        winner: 0,
    };

    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);

            if app.game_over {
                print!("{:?}", app.winner);
                WindowTrait::set_should_close(&mut window, true);
            }
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            app.key_press(key);
        }

        if let Some(Button::Keyboard(key)) = e.release_args() {
            app.key_release(key);
        }
    }
}
