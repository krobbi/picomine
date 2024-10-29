use std::{error, fmt, path::Path};

use image::ImageError;

/// Loads a texture from an image path or a default texture to a mutable slice.
pub fn load_texture(path: &Path, width: usize, height: usize, texture: &mut [u32]) {
    if let Err(error) = try_load_texture(path, width, height, texture) {
        eprintln!("Failed to load texture `{}`: {error}", path.display());
        load_default_texture(width, height, texture);
    }
}

/// Loads a texture from an image path to a mutable slice or returns an error.
fn try_load_texture(
    path: &Path,
    width: usize,
    height: usize,
    texture: &mut [u32],
) -> Result<(), LoadTextureError> {
    let image = image::open(path)?.into_rgb8();

    let (width, height) = (
        u32::try_from(width).unwrap(),
        u32::try_from(height).unwrap(),
    );

    if image.dimensions() != (width, height) {
        return Err(LoadTextureError::UnexpectedImageDimensions);
    }

    for y in 0..height {
        for x in 0..width {
            let (r, g, b) = image.get_pixel(x, y).0.into();

            texture[usize::try_from(x + y * width).unwrap()] =
                (u32::from(r) << 16) | (u32::from(g) << 8) | u32::from(b);
        }
    }

    Ok(())
}

/// Loads a default texture to a mutable slice.
fn load_default_texture(width: usize, height: usize, texture: &mut [u32]) {
    const BORDER_COLOR: u32 = 0x56_2e_73;
    const CENTER_COLOR: u32 = 0xa0_3a_9e;

    for x in 0..width {
        texture[x] = BORDER_COLOR;
        texture[width * height - x - 1] = BORDER_COLOR;
    }

    for y in 1..height - 1 {
        texture[y * width] = BORDER_COLOR;
        texture[(y + 1) * width - 1] = BORDER_COLOR;

        for x in 1..width - 1 {
            texture[x + y * width] = CENTER_COLOR;
        }
    }
}

/// An error encountered while loading a texture.
#[derive(Debug)]
enum LoadTextureError {
    /// An error encountered while loading a texture image.
    Image(ImageError),

    /// An error caused by loading a texture image with unexpected dimensions.
    UnexpectedImageDimensions,
}

impl error::Error for LoadTextureError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::Image(error) => Some(error),
            Self::UnexpectedImageDimensions => None,
        }
    }
}

impl fmt::Display for LoadTextureError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Image(error) => error.fmt(f),
            Self::UnexpectedImageDimensions => write!(f, "texture image has unexpected dimensions"),
        }
    }
}

impl From<ImageError> for LoadTextureError {
    fn from(value: ImageError) -> Self {
        Self::Image(value)
    }
}
