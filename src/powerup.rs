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

pub trait PowerUp {
    fn position(&self) -> &Position;

    fn width(&self) -> f64 {
        POWERUP_SIZE
    }

    fn height(&self) -> f64 {
        POWERUP_SIZE
    }

    fn collectable(&self) -> bool;

    fn set_collectable(&mut self, collectable: bool);

    fn powerup_type(&self) -> PowerUpType;

    fn collided(&self, ball: &Ball) -> bool {
        let p_lx = self.position().x;
        let p_rx = self.position().x + self.width();
        let p_uy = self.position().y;
        let p_dy = self.position().y + self.height();

        let b_lx = ball.position.x - ball.radius;
        let b_rx = ball.position.x + ball.radius;
        let b_y = ball.position.y;

        (p_uy < b_y && b_y < p_dy) && (b_lx < p_rx && b_rx > p_lx)
    }

    fn collect(&mut self, last_hit: usize, player1: &mut dyn Object, player2: &mut dyn Object);
}

pub struct Enlarge {
    position: Position,
    pub collectable: bool,
}

impl Enlarge {
    pub fn new(x: f64, y: f64) -> Self {
        Enlarge {
            position: Position { x, y },
            collectable: true,
        }
    }
}

impl PowerUp for Enlarge {
    fn position(&self) -> &Position {
        &self.position
    }

    fn collectable(&self) -> bool {
        self.collectable
    }

    fn set_collectable(&mut self, collectable: bool) {
        self.collectable = collectable;
    }

    fn powerup_type(&self) -> PowerUpType {
        PowerUpType::Enlarge
    }

    fn collect(&mut self, last_hit: usize, player1: &mut dyn Object, player2: &mut dyn Object) {
        self.set_collectable(false);
        if last_hit == 1 {
            player1.resize_by(20.0);
        } else {
            player2.resize_by(20.0);
        }
    }
}

pub struct Shrink {
    position: Position,
    pub collectable: bool,
}

impl Shrink {
    pub fn new(x: f64, y: f64) -> Self {
        Shrink {
            position: Position { x, y },
            collectable: true,
        }
    }
}

impl PowerUp for Shrink {
    fn position(&self) -> &Position {
        &self.position
    }

    fn collectable(&self) -> bool {
        self.collectable
    }

    fn set_collectable(&mut self, collectable: bool) {
        self.collectable = collectable;
    }

    fn powerup_type(&self) -> PowerUpType {
        PowerUpType::Shrink
    }

    fn collect(&mut self, last_hit: usize, player1: &mut dyn Object, player2: &mut dyn Object) {
        self.set_collectable(false);
        if last_hit == 1 {
            player2.resize_by(-20.0);
        } else {
            player1.resize_by(-20.0);
        }
    }
}

pub struct SpeedUp {
    position: Position,
    pub collectable: bool,
}

impl SpeedUp {
    pub fn new(x: f64, y: f64) -> Self {
        SpeedUp {
            position: Position { x, y },
            collectable: true,
        }
    }
}

impl PowerUp for SpeedUp {
    fn position(&self) -> &Position {
        &self.position
    }

    fn collectable(&self) -> bool {
        self.collectable
    }

    fn set_collectable(&mut self, collectable: bool) {
        self.collectable = collectable;
    }

    fn powerup_type(&self) -> PowerUpType {
        PowerUpType::SpeedUp
    }

    fn collect(&mut self, last_hit: usize, player1: &mut dyn Object, player2: &mut dyn Object) {
        self.set_collectable(false);
        if last_hit == 1 {
            player1.change_speed_by(2.0);
        } else {
            player2.change_speed_by(2.0);
        }
    }
}

pub struct SlowDown {
    position: Position,
    pub collectable: bool,
}

impl SlowDown {
    pub fn new(x: f64, y: f64) -> Self {
        SlowDown {
            position: Position { x, y },
            collectable: true,
        }
    }
}

impl PowerUp for SlowDown {
    fn position(&self) -> &Position {
        &self.position
    }

    fn collectable(&self) -> bool {
        self.collectable
    }

    fn set_collectable(&mut self, collectable: bool) {
        self.collectable = collectable;
    }

    fn powerup_type(&self) -> PowerUpType {
        PowerUpType::SpeedUp
    }

    fn collect(&mut self, last_hit: usize, player1: &mut dyn Object, player2: &mut dyn Object) {
        self.set_collectable(false);
        if last_hit == 1 {
            player2.change_speed_by(-2.0);
        } else {
            player1.change_speed_by(-2.0);
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

        if let Ok(texture) = GlTexture::from_path(Path::new("assets/shrink.png"), &texture_settings)
        {
            sprites.insert(PowerUpType::Shrink, texture);
        }

        if let Ok(texture) =
            GlTexture::from_path(Path::new("assets/speed_up.png"), &texture_settings)
        {
            sprites.insert(PowerUpType::SpeedUp, texture);
        }

        if let Ok(texture) =
            GlTexture::from_path(Path::new("assets/slow_down.png"), &texture_settings)
        {
            sprites.insert(PowerUpType::SlowDown, texture);
        }

        PowerUpSprites { sprites }
    }

    pub fn get(&self, powerup_type: &PowerUpType) -> Option<&GlTexture> {
        self.sprites.get(powerup_type)
    }

    pub fn render(&self, powerup: &dyn PowerUp, c: &graphics::Context, gl: &mut GlGraphics) {
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
