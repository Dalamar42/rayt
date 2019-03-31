use data::colour::Colour;
use image::{ImageBuffer, Rgb, RgbImage};

pub struct Pixel {
    x: u32,
    y: u32,
    colour: Colour,
}

pub struct Image {
    width: u32,
    height: u32,
    pixels: Vec<Pixel>,
}

impl Pixel {
    pub fn new(x: u32, y: u32, colour: Colour) -> Pixel {
        Pixel { x, y, colour }
    }

    fn output_x(&self, _width: u32, _height: u32) -> u32 {
        // Translate into the coordinate system expected by the image crate
        self.x
    }

    fn output_y(&self, _width: u32, height: u32) -> u32 {
        // Translate into the coordinate system expected by the image crate
        height - self.y - 1
    }

    pub fn output_colour(&self) -> Rgb<u8> {
        self.colour.to_rgb()
    }
}

impl Image {
    pub fn new(width: u32, height: u32, pixels: Vec<Pixel>) -> Image {
        Image {
            width,
            height,
            pixels,
        }
    }

    pub fn into_rgb_image(self) -> RgbImage {
        let mut image: RgbImage = ImageBuffer::new(self.width, self.height);

        for pixel in self.pixels {
            image.put_pixel(
                pixel.output_x(self.width, self.height),
                pixel.output_y(self.width, self.height),
                pixel.output_colour(),
            );
        }

        image
    }
}
