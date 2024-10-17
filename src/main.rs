mod camera;
mod tile;
mod window;

use camera::Camera;
use tile::Tile;
use window::{Key, Window};

/// A temporary structure for testing the camera structure.
struct CameraTester {
    /// The camera tester's camera.
    camera: Camera,

    /// The camera tester's X position.
    x: f32,

    /// The camera tester's Y position.
    y: f32,
}

impl CameraTester {
    /// Creates a new camera tester.
    fn new() -> Self {
        Self {
            camera: Camera::new(),
            x: 0.5,
            y: 0.5,
        }
    }

    /// Translates the camera tester and redraws it to a window.
    fn translate(&mut self, x: f32, y: f32, window: &mut Window) {
        self.x += x;
        self.y += y;
        self.camera.set_position(self.x, self.y);

        window.buffer_mut().fill(0x0d_07_09);
        self.camera.draw_tile(Tile::Stone, -2, -2, window);
        self.camera.draw_tile(Tile::Stone, 2, -2, window);
        self.camera.draw_tile(Tile::Grass, 0, 0, window);
        self.camera.draw_tile(Tile::Stone, -2, 2, window);
        self.camera.draw_tile(Tile::Stone, 2, 2, window);
        draw_center_dot(window);
    }
}

/// Opens a window and draws a test scene.
fn main() {
    let mut window = Window::new();
    let mut camera_tester = CameraTester::new();
    camera_tester.translate(0.0, 0.0, &mut window);

    while window.is_open() {
        if window.is_key_pressed(Key::W) {
            camera_tester.translate(0.0, -0.5, &mut window);
        } else if window.is_key_pressed(Key::A) {
            camera_tester.translate(-0.5, 0.0, &mut window);
        } else if window.is_key_pressed(Key::S) {
            camera_tester.translate(0.0, 0.5, &mut window);
        } else if window.is_key_pressed(Key::D) {
            camera_tester.translate(0.5, 0.0, &mut window);
        }

        window.update();
    }
}

/// Draws a dot to the center of a window.
fn draw_center_dot(window: &mut Window) {
    const INDEX: usize = Window::HEIGHT / 2 * Window::WIDTH + Window::WIDTH / 2;
    const COLOR: u32 = 0xf1_f2_f1;

    window.buffer_mut()[INDEX] = COLOR;
}
