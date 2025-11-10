pub trait Object {
    fn resize_by(&mut self, size: f64);
    fn change_speed_by(&mut self, speed: f64);
}
