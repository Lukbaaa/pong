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

pub struct App {
    gl: GlGraphics,
    rotation: f64,
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
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            let transform = c
                .transform
                .trans(x, y)
                .rot_rad(rotation)
                .trans(-25.0, -25.0);

            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("Pong", [800, 800])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
    };

    let mut events = Events::new(EventSettings::new());

    let mut currently_pressed = [false, false, false, false]; // W S UP DOWN

    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }

        if currently_pressed[0] {
            println!("w is pressed");
        }
        if currently_pressed[1] {
            println!("s is pressed");
        }
        if currently_pressed[2] {
            println!("up is pressed");
        }
        if currently_pressed[3] {
            println!("down is pressed");
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            if key == Key::W {
                currently_pressed[0] = true;
                println!("W was pressed");
            }
            if key == Key::S {
                currently_pressed[1] = true;
                println!("S was pressed");
            }
            if key == Key::Up {
                currently_pressed[2] = true;
                println!("Up was pressed");
            }
            if key == Key::Down {
                currently_pressed[3] = true;
                println!("Down was pressed");
            }
        }

        if let Some(Button::Keyboard(key)) = e.release_args() {
            if key == Key::W {
                currently_pressed[0] = false;
                println!("W was released");
            }
            if key == Key::S {
                currently_pressed[1] = false;
                println!("S was released");
            }
            if key == Key::Up {
                currently_pressed[2] = false;
                println!("Up was released");
            }
            if key == Key::Down {
                currently_pressed[3] = false;
                println!("Down was released");
            }
        }
    }
}
