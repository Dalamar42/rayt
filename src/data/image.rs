use crate::data::colour::Colour;
use image::{DynamicImage, ImageBuffer, RgbImage};

pub struct Pixel {
    row: u32,
    col: u32,
    colour: Colour,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Image {
    width: u32,
    height: u32,
    pixels: Vec<Vec<Colour>>,
}

impl Pixel {
    pub fn new(row: u32, col: u32, colour: Colour) -> Pixel {
        Pixel { row, col, colour }
    }
}

impl Image {
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn get_pixel(&self, row: u32, col: u32) -> &Colour {
        &self.pixels[row as usize][col as usize]
    }

    pub fn into_rgb_image(self) -> RgbImage {
        let mut image: RgbImage = ImageBuffer::new(self.width, self.height);

        for (row_idx, row) in self.pixels.iter().enumerate() {
            for (col_idx, colour) in row.iter().enumerate() {
                // Translate into the coordinate system expected by the image crate
                image.put_pixel(
                    col_idx as u32,
                    self.height - row_idx as u32 - 1,
                    colour.to_rgb(),
                );
            }
        }

        image
    }
}

impl From<&Vec<Pixel>> for Image {
    fn from(pixels: &Vec<Pixel>) -> Self {
        let width = pixels.iter().map(|pixel| pixel.col).max().unwrap() + 1;
        let height = pixels.iter().map(|pixel| pixel.row).max().unwrap() + 1;

        let mut pixel_matrix: Vec<Vec<Colour>> =
            vec![vec![Colour::new(1.0, 1.0, 1.0); width as usize]; height as usize];
        for pixel in pixels {
            pixel_matrix[pixel.row as usize][pixel.col as usize] = pixel.colour;
        }

        Image {
            width,
            height,
            pixels: pixel_matrix,
        }
    }
}

impl From<&RgbImage> for Image {
    fn from(image: &RgbImage) -> Self {
        let (width, height) = image.dimensions();

        let mut pixels: Vec<Pixel> = vec![];
        for x in 0..width {
            for y in 0..height {
                let rgb = image.get_pixel(x, y);
                let colour = Colour::from(rgb);

                // Translate from the coordinate system expected by the image crate
                let pixel = Pixel::new(height - y - 1, x, colour);
                pixels.push(pixel);
            }
        }

        Image::from(&pixels)
    }
}

impl From<&DynamicImage> for Image {
    fn from(image: &DynamicImage) -> Self {
        let image = image.to_rgb();
        Image::from(&image)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    fn assert_colour_eq(actual: &Colour, expected: &Colour) {
        assert_approx_eq!(actual.r(), expected.r(), 0.01);
        assert_approx_eq!(actual.g(), expected.g(), 0.01);
        assert_approx_eq!(actual.b(), expected.b(), 0.01);
    }

    #[test]
    fn test_image_from_pixels() {
        let pixels = vec![
            Pixel::new(0, 0, Colour::new(0.0, 0.0, 0.0)),
            Pixel::new(0, 1, Colour::new(0.0, 0.0, 1.0)),
            Pixel::new(1, 0, Colour::new(0.0, 1.0, 0.0)),
            Pixel::new(1, 1, Colour::new(1.0, 0.0, 0.0)),
        ];
        let image = Image::from(&pixels);

        assert_eq!(image.width(), 2);
        assert_eq!(image.height(), 2);

        assert_colour_eq(image.get_pixel(0, 0), &Colour::new(0.0, 0.0, 0.0));
        assert_colour_eq(image.get_pixel(0, 1), &Colour::new(0.0, 0.0, 1.0));
        assert_colour_eq(image.get_pixel(1, 0), &Colour::new(0.0, 1.0, 0.0));
        assert_colour_eq(image.get_pixel(1, 1), &Colour::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn test_convert_image_to_rgb_image_and_back() {
        let pixels = vec![
            Pixel::new(0, 0, Colour::new(0.0, 0.0, 0.0)),
            Pixel::new(0, 1, Colour::new(0.0, 0.0, 1.0)),
            Pixel::new(1, 0, Colour::new(0.0, 1.0, 0.0)),
            Pixel::new(1, 1, Colour::new(1.0, 0.0, 0.0)),
        ];
        let image = Image::from(&pixels);

        let rgb_image = image.into_rgb_image();
        let image = Image::from(&rgb_image);

        assert_eq!(image.width(), 2);
        assert_eq!(image.height(), 2);

        assert_colour_eq(image.get_pixel(0, 0), &Colour::new(0.0, 0.0, 0.0));
        assert_colour_eq(image.get_pixel(0, 1), &Colour::new(0.0, 0.0, 1.0));
        assert_colour_eq(image.get_pixel(1, 0), &Colour::new(0.0, 1.0, 0.0));
        assert_colour_eq(image.get_pixel(1, 1), &Colour::new(1.0, 0.0, 0.0));
    }
}
