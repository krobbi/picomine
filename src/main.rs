// Disable console in release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod camera;
mod chunk;
mod tile;
mod window;
mod world;

use camera::Camera;
use tile::Tile;
use window::{Key, Window};
use world::World;

/// Opens a window and draws a world with a camera.
fn main() {
    const CAMERA_STEP: f32 = 1.0;
    const CENTER_INDEX: usize = Window::WIDTH / 2 + Window::HEIGHT / 2 * Window::WIDTH;
    const CLEAR_COLOR: u32 = 0x0d_07_09;

    let mut window = Window::new();
    let mut world = World::new();
    let mut camera = Camera::new();
    let mut x = 0.5;
    let mut y = 0.5;

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

        if window.is_key_pressed(Key::Up) {
            let (x, y) = world_to_tile_position(x, y);
            world.set_tile(x, y, Tile::Stone);
        } else if window.is_key_pressed(Key::Down) {
            let (x, y) = world_to_tile_position(x, y);
            world.set_tile(x, y, Tile::Grass);
        }

        camera.set_position(x, y);
        window.buffer_mut().fill(CLEAR_COLOR);
        camera.draw_world(&mut world, &mut window);
        window.buffer_mut()[CENTER_INDEX] = CLEAR_COLOR;
        window.update();
    }
}

/// Returns a world position as a tile position.
fn world_to_tile_position(x: f32, y: f32) -> (i32, i32) {
    #[allow(clippy::cast_possible_truncation)]
    (x.floor() as i32, y.floor() as i32)
}
