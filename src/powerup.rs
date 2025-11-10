use opengl_graphics::GlGraphics;
use opengl_graphics::Texture as GlTexture;
use piston_window::{DrawState, Filter, Image, TextureSettings};
use std::collections::HashMap;
use std::path::Path;

use crate::Ball;
use crate::Position;
use crate::object::Object;

const POWERUP_SIZE: f64 = 32.0;

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
    pub collectable: bool,
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

    pub fn collect<T: Object>(&mut self, object: &mut T) {
        self.collectable = false;

        match self.powerup_type {
            PowerUpType::Enlarge => {
                object.resize_by(20.0);
            }
            PowerUpType::Shrink => println!(""),
            PowerUpType::SpeedUp => println!(""),
            PowerUpType::SlowDown => println!(""),
        }
    }

    pub fn countdown_and_revoke(&mut self, dt: f64) {
        self.time_left -= dt;

        if self.time_left >= 0.0 {
            match self.powerup_type {
                PowerUpType::Enlarge => {
                    println!("shrink");
                }
                PowerUpType::Shrink => println!(""),
                PowerUpType::SpeedUp => println!(""),
                PowerUpType::SlowDown => println!(""),
            }
        }
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
