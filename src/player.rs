use crate::Ball;
use crate::Position;
use crate::constants::HEIGHT;
use crate::object::Object;

pub struct Player {
    pub height: f64,
    pub width: f64,
    pub speed: f64,
    pub position: Position,
}

impl Player {
    pub fn collided(&self, ball: &Ball) -> bool {
        let p_lx = self.position.x;
        let p_rx = self.position.x + self.width;
        let p_uy = self.position.y;
        let p_dy = self.position.y + self.height;
        let b_lx = ball.position.x - ball.radius;
        let b_rx = ball.position.x + ball.radius;
        let b_y = ball.position.y;

        (p_uy < b_y && b_y < p_dy) && (b_lx < p_rx && b_rx > p_lx)
    }

    pub fn collision_point(&self, ball: &Ball) -> f64 {
        let b_dy = ball.position.y - self.position.y;
        let b_ry = b_dy / self.height;

        b_ry * 2.0 - 1.0
    }

    pub fn prevent_out_of_bounds(&mut self) {
        if self.position.y < 0.0 {
            self.position.y += self.speed;
        }
        if self.position.y > HEIGHT - self.height {
            self.position.y -= self.speed;
        }
    }

    pub fn reset(&mut self) {
        self.height = 80.0;
        self.width = 16.0;
        self.speed = 5.0;
        self.position.x = 50.0;
        self.position.y = HEIGHT / 2.0 - self.height / 2.0;
    }
}

impl Object for Player {
    fn resize_by(&mut self, size: f64) {
        self.height += size;
        self.position.y -= size / 2.0;
    }

    fn change_speed_by(&mut self, speed: f64) {
        self.speed -= speed;
    }
}
