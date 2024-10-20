mod camera;
mod chunk;
mod tile;
mod window;
mod world;

use camera::Camera;
use window::{Key, Window};
use world::World;

/// Opens a window and draws a world with a camera.
fn main() {
    const CAMERA_STEP: f32 = 1.0 / 16.0;
    const CENTER_INDEX: usize = Window::WIDTH / 2 + Window::HEIGHT / 2 * Window::WIDTH;

    let mut window = Window::new();
    let mut world = World::new();
    let mut camera = Camera::new();
    let mut x = 0.0;
    let mut y = 0.0;

    while window.is_open() {
        if window.is_key_pressed(Key::W) {
            y -= CAMERA_STEP;
        } else if window.is_key_pressed(Key::A) {
            x -= CAMERA_STEP;
        } else if window.is_key_pressed(Key::S) {
            y += CAMERA_STEP;
        } else if window.is_key_pressed(Key::D) {
            x += CAMERA_STEP;
        }

        camera.set_position(x, y);
        window.buffer_mut().fill(0x0d_07_09);
        camera.draw_world(&mut world, &mut window);
        window.buffer_mut()[CENTER_INDEX] = 0xf1_f2_f1;
        window.update();
    }
}
