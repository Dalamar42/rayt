use data::colour::Colour;
use image::{DynamicImage, ImageBuffer, Rgb, RgbImage};

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
            // Translate into the coordinate system expected by the image crate
            image.put_pixel(pixel.x, self.height - pixel.y - 1, pixel.output_colour());
        }

        image
    }
}

impl From<&DynamicImage> for Image {
    fn from(image: &DynamicImage) -> Self {
        let image = image.to_rgb();
        let (width, height) = image.dimensions();

        let mut pixels: Vec<Pixel> = vec![];
        for x in 0..width {
            for y in 0..height {
                let rgb = image.get_pixel(x, y);
                let colour = Colour::from(rgb);

                // Translate into the coordinate system expected by the image crate
                let pixel = Pixel::new(x, height - y - 1, colour);
                pixels.push(pixel);
            }
        }

        Image::new(width, height, pixels)
    }
}
