use crate::HEIGHT;
use crate::Position;
use crate::object::Object;

pub struct Ball {
    pub radius: f64,
    pub speed: f64,
    pub angle: f64,
    pub position: Position,
    pub last_hit: usize,
}

impl Ball {
    pub fn check_and_handle_vertical_collision(&mut self) {
        if self.position.y - self.radius < 0.0 || self.position.y + self.radius > HEIGHT {
            self.angle = -self.angle;
        }
    }
}

impl Object for Ball {
    fn resize_by(&mut self, size: f64) {
        self.radius += size;
    }

    fn change_speed_by(&mut self, speed: f64) {
        self.speed += speed;
    }
}
