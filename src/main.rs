// Disable console in release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod camera;
mod chunk;
mod tile;
mod window;
mod world;

use camera::Camera;
use tile::Tile;
use window::{Key, MouseButton, Window};
use world::World;

/// Opens a window and draws a world with a camera.
fn main() {
    let mut window = Window::new();
    let mut world = World::new();
    let mut camera = Camera::new();
    let mut x = 16.0;
    let mut y = 16.5;

    while window.is_open() {
        if window.is_key_pressed(Key::W) {
            y -= 1.0;
        } else if window.is_key_pressed(Key::A) {
            x -= 1.0;
        } else if window.is_key_pressed(Key::S) {
            y += 1.0;
        } else if window.is_key_pressed(Key::D) {
            x += 1.0;
        }

        camera.set_position(x, y);

        if window.is_mouse_button_down(MouseButton::Left) {
            set_mouse_tile(&window, camera, &mut world, Tile::Grass);
        } else if window.is_mouse_button_down(MouseButton::Right) {
            set_mouse_tile(&window, camera, &mut world, Tile::Stone);
        }

        window.buffer_mut().fill(0x0d_07_09);
        camera.draw_world(&mut world, &mut window);
        window.update();
    }
}

/// Sets a tile at the current mouse position.
fn set_mouse_tile(window: &Window, camera: Camera, world: &mut World, tile: Tile) {
    let (x, y) = window.get_mouse_position();
    let (x, y) = camera.screen_to_tile_position(x, y);
    world.set_tile(x, y, tile);
}
