use minifb::{Scale, WindowOptions};

/// A game window for rendering.
pub struct Window {
    /// The window's inner window.
    inner: minifb::Window,

    /// The window's framebuffer.
    buffer: Vec<u32>,
}

impl Window {
    /// A window's width in pixels.
    const WIDTH: usize = 320;

    /// A window's height in pixels.
    const HEIGHT: usize = 180;

    /// Create a new window.
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

    /// Get whether the window is open.
    pub fn is_open(&self) -> bool {
        self.inner.is_open()
    }

    /// Update the window.
    pub fn update(&mut self) {
        self.inner
            .update_with_buffer(&self.buffer, Self::WIDTH, Self::HEIGHT)
            .unwrap();
    }
}
