// Disable console in release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod camera;
mod chunk;
mod resources;
mod terrain;
mod tile;
mod window;
mod world;

use camera::Camera;
use tile::Tile;
use window::{Key, MouseButton, Window};
use world::World;

/// Opens a window and draws a world with a camera.
fn main() {
    const WORLD_SEED: u32 = 0xd981_c964;

    Tile::load_textures();
    let mut window = Window::new();
    let mut world = World::new(WORLD_SEED);
    let mut camera = Camera::new();
    let (mut player_x, mut player_y) = (0.5, 0.5);
    let mut held_tile = Tile::Stone;

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

            (player_x, player_y) = (player_x + joy_x * velocity, player_y + joy_y * velocity);
            camera.set_position(player_x, player_y);
        }

        if window.is_key_down(Key::Key1) {
            held_tile = Tile::Grass;
        } else if window.is_key_down(Key::Key2) {
            held_tile = Tile::Stone;
        } else if window.is_key_down(Key::Key3) {
            held_tile = Tile::Sand;
        } else if window.is_key_down(Key::Key4) {
            held_tile = Tile::Water;
        }

        if window.is_mouse_button_down(MouseButton::Right) {
            let (mouse_x, mouse_y) = window.get_mouse_position();
            let (mouse_x, mouse_y) = camera.screen_to_tile_position(mouse_x, mouse_y);
            world.set_tile(mouse_x, mouse_y, held_tile);
        }

        camera.draw_world(&mut world, &mut window);

        {
            const BORDER_COLOR: u32 = 0x0d_07_09;

            let buffer = window.buffer_mut();
            let texture = held_tile.texture();
            let mut buffer_index = Window::WIDTH + 2;
            let mut texture_index = 0;
            buffer[buffer_index - 1..=buffer_index + Tile::WIDTH].fill(BORDER_COLOR);

            for _ in 0..Tile::HEIGHT {
                buffer_index += Window::WIDTH;
                buffer[buffer_index - 1] = BORDER_COLOR;

                buffer[buffer_index..buffer_index + Tile::WIDTH]
                    .copy_from_slice(&texture[texture_index..texture_index + Tile::WIDTH]);

                buffer[buffer_index + Tile::WIDTH] = BORDER_COLOR;
                texture_index += Tile::WIDTH;
            }

            buffer_index += Window::WIDTH;
            buffer[buffer_index - 1..=buffer_index + Tile::WIDTH].fill(BORDER_COLOR);
        }

        window.update();
    }
}
