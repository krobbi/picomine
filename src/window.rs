pub use minifb::{Key, MouseButton};

use std::time::Instant;

use minifb::{MouseMode, Scale, WindowOptions};

/// A game window for timing, input, and drawing.
pub struct Window {
    /// The window's inner window.
    inner: minifb::Window,

    /// The window's framebuffer.
    buffer: Vec<u32>,

    /// The instant of the start of the current frame.
    update_instant: Instant,

    /// The duration of the previous frame in seconds.
    delta: f32,
}

impl Window {
    /// A window's width in pixels.
    pub const WIDTH: usize = 320;

    /// A window's height in pixels.
    pub const HEIGHT: usize = 180;

    /// Creates and opens a new window.
    pub fn new() -> Self {
        let inner = minifb::Window::new(
            "PicoMine",
            Self::WIDTH,
            Self::HEIGHT,
            WindowOptions {
                scale: Scale::X4,
                ..Default::default()
            },
        )
        .unwrap_or_else(|e| panic!("{e}"));

        Self {
            inner,
            buffer: vec![0; Self::WIDTH * Self::HEIGHT],
            update_instant: Instant::now(),
            delta: 0.0,
        }
    }

    /// Returns the duration of the previous frame in seconds.
    pub fn get_delta(&self) -> f32 {
        self.delta
    }

    /// Returns the current mouse position clamped to screen space.
    pub fn get_mouse_position(&self) -> (f32, f32) {
        self.inner.get_mouse_pos(MouseMode::Clamp).unwrap()
    }

    /// Returns whether the window has not been closed.
    pub fn is_open(&self) -> bool {
        self.inner.is_open()
    }

    /// Returns whether a key is currently being held down.
    pub fn is_key_down(&self, key: Key) -> bool {
        self.inner.is_key_down(key)
    }

    /// Returns whether a mouse button is currently being held down.
    pub fn is_mouse_button_down(&self, mouse_button: MouseButton) -> bool {
        self.inner.get_mouse_down(mouse_button)
    }

    /// Returns the window's framebuffer as a mutable slice.
    pub fn buffer_mut(&mut self) -> &mut [u32] {
        &mut self.buffer
    }

    /// Updates the window with a new frame from its framebuffer.
    pub fn update(&mut self) {
        self.inner
            .update_with_buffer(&self.buffer, Self::WIDTH, Self::HEIGHT)
            .unwrap();

        let now = Instant::now();
        self.delta = now.duration_since(self.update_instant).as_secs_f32();
        self.update_instant = now;
    }
}
