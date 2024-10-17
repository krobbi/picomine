pub use minifb::Key;

use minifb::{KeyRepeat, Scale, WindowOptions};

/// A game window for keyboard input and drawing.
pub struct Window {
    /// The window's inner window.
    inner: minifb::Window,

    /// The window's framebuffer.
    buffer: Vec<u32>,
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

        let buffer = vec![0; Self::WIDTH * Self::HEIGHT];
        Self { inner, buffer }
    }

    /// Returns whether the window has not been closed.
    pub fn is_open(&self) -> bool {
        self.inner.is_open()
    }

    /// Returns whether a key started being pressed on the current frame.
    pub fn is_key_pressed(&self, key: Key) -> bool {
        self.inner.is_key_pressed(key, KeyRepeat::No)
    }

    /// Returns the window's framebuffer as a mutable slice.
    pub fn buffer_mut(&mut self) -> &mut [u32] {
        &mut self.buffer
    }

    /// Updates the window with its framebuffer.
    pub fn update(&mut self) {
        self.inner
            .update_with_buffer(&self.buffer, Self::WIDTH, Self::HEIGHT)
            .unwrap();
    }
}