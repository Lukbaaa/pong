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
use rand::Rng;
use rand::seq::IndexedRandom;
use std::collections::HashSet;
use std::f64::consts::PI;

mod number_renderer;
use number_renderer::NumberRenderer;

mod position;
use position::Position;

mod powerup;
use powerup::*;

mod player;
use player::Player;

mod ball;
use ball::Ball;

mod constants;
use constants::*;

mod object;

pub struct App {
    gl: GlGraphics,
    number_renderer: NumberRenderer,
    powerup_sprites: PowerUpSprites,
    active_powerups: Vec<Box<dyn PowerUp>>,
    pressed_keys: HashSet<Key>,
    player1: Player,
    player2: Player,
    kick_off: usize,
    ball: Ball,
    game_over: bool,
    score: [u32; 2],
    winner: usize,
    time_to_spawn_power_up: f64,
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
            let to_player1 = [self.player1.width, self.player1.height];
            let to_player2 = [self.player2.width, self.player2.height];

            ellipse(BLACK, ball, ball_transform, gl);
            rectangle_from_to(BLACK, from, to_player1, player1_transform, gl);
            rectangle_from_to(BLACK, from, to_player2, player2_transform, gl);

            self.number_renderer.render(
                self.score[0],
                WIDTH / 4.0,
                50.0,
                100.0,
                [0.0, 0.0, 0.0],
                &c,
                gl,
            );

            self.number_renderer.render(
                self.score[1],
                3.0 * WIDTH / 4.0,
                50.0,
                100.0,
                [0.0, 0.0, 0.0],
                &c,
                gl,
            );

            for powerup in &self.active_powerups {
                if powerup.collectable() {
                    self.powerup_sprites.render(powerup.as_ref(), &c, gl);
                }
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        for key in &self.pressed_keys {
            match key {
                Key::W => self.player1.position.y -= self.player1.speed,
                Key::S => self.player1.position.y += self.player1.speed,
                Key::Up => self.player2.position.y -= self.player2.speed,
                Key::Down => self.player2.position.y += self.player2.speed,
                _ => {}
            }
            let player_num = match key {
                Key::W | Key::S => 1,
                Key::Up | Key::Down => 2,
                _ => 0,
            };
            if player_num == self.kick_off {
                self.ball.speed = 3.0;
            }
        }

        self.player1.prevent_out_of_bounds();
        self.player2.prevent_out_of_bounds();

        self.ball.position.x += self.ball.angle.cos() * self.ball.speed;
        self.ball.position.y -= self.ball.angle.sin() * self.ball.speed;

        self.ball.check_and_handle_vertical_collision();

        if self.player1.collided(&self.ball) {
            let collision_point = self.player1.collision_point(&self.ball);
            self.ball.angle = -(75.0 * PI / 180.0) * collision_point;
            self.ball.speed = 3.0 + collision_point.abs() * 3.0;
            self.ball.last_hit = 1;
        }

        if self.player2.collided(&self.ball) {
            let collision_point = self.player2.collision_point(&self.ball);
            self.ball.angle = PI + (75.0 * PI / 180.0) * collision_point;
            self.ball.speed = 3.0 + collision_point.abs() * 3.0;
            self.ball.last_hit = 2;
        }

        if self.ball.position.x < self.player1.position.x {
            self.scored(2);
        }
        if self.ball.position.x > self.player2.position.x {
            self.scored(1);
        }

        if self.score[0] == 10 {
            self.winner = 1;
            self.game_over = true;
        } else if self.score[1] == 10 {
            self.winner = 2;
            self.game_over = true;
        }

        self.time_to_spawn_power_up -= args.dt;
        if self.time_to_spawn_power_up <= 0.0 {
            self.spawn_power_up();
            self.time_to_spawn_power_up = 10.0;
        }

        let mut collected_indices = Vec::new();

        for (i, powerup) in self.active_powerups.iter().enumerate() {
            if powerup.collectable() && powerup.collided(&self.ball) {
                collected_indices.push(i);
            }
        }

        for i in collected_indices.into_iter().rev() {
            self.active_powerups[i].collect(
                self.ball.last_hit,
                &mut self.player1,
                &mut self.player2,
            );
            self.active_powerups.remove(i);
        }
    }

    fn key_press(&mut self, key: Key) {
        self.pressed_keys.insert(key);
    }

    fn key_release(&mut self, key: Key) {
        self.pressed_keys.remove(&key);
    }

    fn scored(&mut self, scoring_player: usize) {
        self.ball.speed = 0.0;
        self.score[scoring_player - 1] += 1;
        self.kick_off = 3 - scoring_player;

        self.player1.reset();
        self.player2.reset();

        let (ball_x, angle) = if self.kick_off == 1 {
            (self.player1.position.x + self.player1.width + 20.0, 0.0)
        } else {
            (self.player2.position.x - 20.0, PI)
        };

        self.ball.position.x = ball_x;
        self.ball.position.y = HEIGHT / 2.0;
        self.ball.angle = angle;

        self.active_powerups.clear();
    }

    fn spawn_power_up(&mut self) {
        let mut rng = rand::rng();
        let types = [
            PowerUpType::Enlarge,
            PowerUpType::Shrink,
            PowerUpType::SpeedUp,
            PowerUpType::SlowDown,
        ];

        let rnd_type = types.choose(&mut rng).unwrap();

        let spawn_x = rng.random_range(SPRITE_SPAWN_MARGIN..WIDTH - SPRITE_SPAWN_MARGIN);
        let spawn_y = rng.random_range(50.0..HEIGHT - 50.0);

        match rnd_type {
            PowerUpType::Enlarge => self
                .active_powerups
                .push(Box::new(Enlarge::new(spawn_x, spawn_y))),
            PowerUpType::Shrink => self
                .active_powerups
                .push(Box::new(Shrink::new(spawn_x, spawn_y))),
            PowerUpType::SpeedUp => self
                .active_powerups
                .push(Box::new(SpeedUp::new(spawn_x, spawn_y))),
            PowerUpType::SlowDown => self
                .active_powerups
                .push(Box::new(SlowDown::new(spawn_x, spawn_y))),
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
        last_hit: 1,
    };

    let mut app = App {
        gl: GlGraphics::new(opengl),
        number_renderer: NumberRenderer::new(),
        powerup_sprites: PowerUpSprites::new(),
        active_powerups: Vec::new(),
        pressed_keys: HashSet::new(),
        player1,
        player2,
        kick_off: 1,
        ball,
        game_over: false,
        score: [0, 0],
        winner: 0,
        time_to_spawn_power_up: 10.0,
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
