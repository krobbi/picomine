// Disable console in release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod camera;
mod chunk;
mod resources;
mod tile;
mod window;
mod world;

use camera::Camera;
use tile::Tile;
use window::{Key, MouseButton, Window};
use world::World;

/// Opens a window and draws a world with a camera.
fn main() {
    Tile::load_textures();

    let mut window = Window::new();
    let mut world = World::new();
    let mut camera = Camera::new();
    let (mut x, mut y) = (15.5, 15.5);

    while window.is_open() {
        {
            const SPEED: f32 = 6.0;
            const SLOW_SPEED: f32 = 0.5;

            let (mut joy_x, mut joy_y) = (
                f32::from(window.is_key_down(Key::D)) - f32::from(window.is_key_down(Key::A)),
                f32::from(window.is_key_down(Key::S)) - f32::from(window.is_key_down(Key::W)),
            );

            if joy_x != 0.0 && joy_y != 0.0 {
                const FRAC_1_SQRT_2: f32 = std::f32::consts::FRAC_1_SQRT_2;

                (joy_x, joy_y) = (joy_x * FRAC_1_SQRT_2, joy_y * FRAC_1_SQRT_2);
            }

            let velocity = if window.is_key_down(Key::LeftShift) {
                SLOW_SPEED
            } else {
                SPEED
            } * window.get_delta();

            (x, y) = (x + joy_x * velocity, y + joy_y * velocity);
            camera.set_position(x, y);
        }

        if window.is_mouse_button_down(MouseButton::Left) {
            set_mouse_tile(&window, camera, &mut world, Tile::Grass);
        } else if window.is_mouse_button_down(MouseButton::Right) {
            set_mouse_tile(&window, camera, &mut world, Tile::Stone);
        }

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
