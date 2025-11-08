use glutin_window::GlutinWindow as Window;
use opengl_graphics::Texture as GlTexture;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::Button;
use piston::Key;
use piston::PressEvent;
use piston::ReleaseEvent;
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::Window as WindowTrait;
use piston::window::WindowSettings;
use piston_window::{DrawState, Filter, Image, TextureSettings};
use rand::Rng;
use rand::seq::IndexedRandom;
use std::collections::HashMap;
use std::collections::HashSet;
use std::f64::consts::PI;
use std::path::Path;

mod number_renderer;
use number_renderer::NumberRenderer;

const WIDTH: f64 = 800f64;
const HEIGHT: f64 = 800f64;
const POWERUP_SIZE: f64 = 32.0;
const SPRITE_SPAWN_MARGIN: f64 = 150.0;

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub enum PowerUpType {
    Enlarge,
    Shrink,
    SpeedUp,
    SlowDown,
}

pub struct PowerUp {
    position: Position,
    powerup_type: PowerUpType,
    collectable: bool,
    time_left: f64,
}

impl PowerUp {
    pub fn new(x: f64, y: f64, powerup_type: PowerUpType) -> Self {
        PowerUp {
            position: Position { x, y },
            powerup_type,
            collectable: true,
            time_left: 5.0,
        }
    }

    pub fn position(&self) -> &Position {
        &self.position
    }

    pub fn powerup_type(&self) -> PowerUpType {
        self.powerup_type
    }

    pub fn width(&self) -> f64 {
        POWERUP_SIZE
    }

    pub fn height(&self) -> f64 {
        POWERUP_SIZE
    }

    pub fn collided(&self, ball: &Ball) -> bool {
        let p_lx = self.position.x;
        let p_rx = self.position.x + self.width();
        let p_uy = self.position.y;
        let p_dy = self.position.y + self.height();

        let b_lx = ball.position.x - ball.radius;
        let b_rx = ball.position.x + ball.radius;
        let b_y = ball.position.y;

        (p_uy < b_y && b_y < p_dy) && (b_lx < p_rx && b_rx > p_lx)
    }

    pub fn collect(&mut self, app: &App) {
        self.collectable = false;
    }
}

pub struct PowerUpSprites {
    sprites: HashMap<PowerUpType, GlTexture>,
}

impl Default for PowerUpSprites {
    fn default() -> Self {
        Self::new()
    }
}

impl PowerUpSprites {
    pub fn new() -> Self {
        let texture_settings = TextureSettings::new()
            .filter(Filter::Nearest)
            .mipmap(Filter::Nearest);

        let mut sprites = HashMap::new();

        if let Ok(texture) =
            GlTexture::from_path(Path::new("assets/enlarge.png"), &texture_settings)
        {
            sprites.insert(PowerUpType::Enlarge, texture);
        }
        // sprites.insert(PowerUpType::Shrink, GlTexture::from_path(...));
        // sprites.insert(PowerUpType::SpeedUp, GlTexture::from_path(...));
        // sprites.insert(PowerUpType::SlowDown, GlTexture::from_path(...));

        PowerUpSprites { sprites }
    }

    pub fn get(&self, powerup_type: &PowerUpType) -> Option<&GlTexture> {
        self.sprites.get(powerup_type)
    }

    pub fn render(&self, powerup: &PowerUp, c: &graphics::Context, gl: &mut GlGraphics) {
        if let Some(sprite) = self.get(&powerup.powerup_type()) {
            Image::new()
                .rect([
                    powerup.position().x,
                    powerup.position().y,
                    powerup.width(),
                    powerup.height(),
                ])
                .draw(sprite, &DrawState::default(), c.transform, gl);
        }
    }
}

pub struct App {
    gl: GlGraphics,
    number_renderer: NumberRenderer,
    powerup_sprites: PowerUpSprites,
    active_powerups: Vec<PowerUp>,
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
    last_hit: usize,
}

impl Player {
    fn collided(&self, ball: &Ball) -> bool {
        let p_lx = self.position.x;
        let p_rx = self.position.x + self.width;
        let p_uy = self.position.y;
        let p_dy = self.position.y + self.height;
        let b_lx = ball.position.x - ball.radius;
        let b_rx = ball.position.x + ball.radius;
        let b_y = ball.position.y;

        (p_uy < b_y && b_y < p_dy) && (b_lx < p_rx && b_rx > p_lx)
    }

    fn collision_point(&self, ball: &Ball) -> f64 {
        let b_dy = ball.position.y - self.position.y;
        let b_ry = b_dy / self.height;

        b_ry * 2.0 - 1.0
    }

    fn prevent_out_of_bounds(&mut self) {
        if self.position.y < 0.0 {
            self.position.y += self.speed;
        }
        if self.position.y > HEIGHT - self.height {
            self.position.y -= self.speed;
        }
    }
}

impl Ball {
    fn check_and_handle_vertical_collision(&mut self) {
        if self.position.y - self.radius < 0.0 || self.position.y + self.radius > HEIGHT {
            self.angle = -self.angle;
        }
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
                if powerup.collectable {
                    self.powerup_sprites.render(powerup, &c, gl);
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

        for powerup in &self.active_powerups {
            if powerup.collided(&self.ball) {
                powerup.collect(self);
            }
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

        let (ball_x, angle) = if self.kick_off == 1 {
            (self.player1.position.x + self.player1.width + 20.0, 0.0)
        } else {
            (self.player2.position.x - 20.0, PI)
        };

        self.ball.position.x = ball_x;
        self.ball.position.y = HEIGHT / 2.0;
        self.ball.angle = angle;

        self.player1.position.y = HEIGHT / 2.0 - 40.0;
        self.player2.position.y = HEIGHT / 2.0 - 40.0;
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

        let powerup = PowerUp::new(spawn_x, spawn_y, PowerUpType::Enlarge);

        self.active_powerups.push(powerup);
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
