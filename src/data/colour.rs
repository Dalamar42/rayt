use image::Rgb;
use std::cmp::min;
use std::iter::Sum;
use std::ops;

const RGB_MULT: f64 = 255.99;
const RGB_MAX: u64 = 255;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Colour {
    r: f64,
    g: f64,
    b: f64,
}

impl Colour {
    pub fn new(r: f64, g: f64, b: f64) -> Colour {
        Colour { r, g, b }
    }

    pub fn r(&self) -> f64 {
        self.r
    }

    pub fn g(&self) -> f64 {
        self.g
    }

    pub fn b(&self) -> f64 {
        self.b
    }

    pub fn len(&self) -> f64 {
        (&self).len_squared().sqrt()
    }

    pub fn len_squared(&self) -> f64 {
        self.r * self.r + self.g * self.g + self.b * self.b
    }

    pub fn unit_vector(&self) -> Colour {
        let k = 1.0 / self.len();
        Colour {
            r: self.r * k,
            g: self.g * k,
            b: self.b * k,
        }
    }

    pub fn dot(lhs: &Colour, rhs: &Colour) -> f64 {
        lhs.r * rhs.r + lhs.g * rhs.g + lhs.b * rhs.b
    }

    pub fn to_rgb(&self) -> Rgb<u8> {
        assert!(0.0 <= self.r, "Found negative r {}", self.r);
        assert!(0.0 <= self.g, "Found negative g {}", self.g);
        assert!(0.0 <= self.b, "Found negative b {}", self.b);

        let r = (RGB_MULT * self.r) as u64;
        let g = (RGB_MULT * self.g) as u64;
        let b = (RGB_MULT * self.b) as u64;

        // Lights can be brighter than (1.0, 1.0, 1.0) so we must cap to max value for RGB
        let r = min(r, RGB_MAX) as u8;
        let g = min(g, RGB_MAX) as u8;
        let b = min(b, RGB_MAX) as u8;

        Rgb([r, g, b])
    }

    pub fn gamma_2(self) -> Colour {
        Colour {
            r: self.r.sqrt(),
            g: self.g.sqrt(),
            b: self.b.sqrt(),
        }
    }
}

impl From<&Rgb<u8>> for Colour {
    fn from(rgb: &Rgb<u8>) -> Self {
        Colour {
            r: f64::from(rgb[0]) / RGB_MULT,
            g: f64::from(rgb[1]) / RGB_MULT,
            b: f64::from(rgb[2]) / RGB_MULT,
        }
    }
}

impl Sum<Colour> for Colour {
    fn sum<I: Iterator<Item = Colour>>(iter: I) -> Colour {
        let mut sum = Colour {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        };
        for colour in iter {
            sum = sum + colour;
        }
        sum
    }
}

fn add_colours(lhs: &Colour, rhs: &Colour) -> Colour {
    Colour {
        r: lhs.r + rhs.r,
        g: lhs.g + rhs.g,
        b: lhs.b + rhs.b,
    }
}

fn mul_colours(lhs: &Colour, rhs: &Colour) -> Colour {
    Colour {
        r: lhs.r * rhs.r,
        g: lhs.g * rhs.g,
        b: lhs.b * rhs.b,
    }
}

fn add_colour_and_scalar(colour: &Colour, scalar: f64) -> Colour {
    Colour {
        r: colour.r + scalar,
        g: colour.g + scalar,
        b: colour.b + scalar,
    }
}

fn mul_colour_and_scalar(colour: &Colour, scalar: f64) -> Colour {
    Colour {
        r: colour.r * scalar,
        g: colour.g * scalar,
        b: colour.b * scalar,
    }
}

fn div_colour_and_scalar(colour: &Colour, scalar: f64) -> Colour {
    Colour {
        r: colour.r / scalar,
        g: colour.g / scalar,
        b: colour.b / scalar,
    }
}

impl ops::Add<&Colour> for &Colour {
    type Output = Colour;

    fn add(self, rhs: &Colour) -> Colour {
        add_colours(self, rhs)
    }
}

impl ops::Add<Colour> for &Colour {
    type Output = Colour;

    fn add(self, rhs: Colour) -> Colour {
        add_colours(self, &rhs)
    }
}

impl ops::Add<&Colour> for Colour {
    type Output = Colour;

    fn add(self, rhs: &Colour) -> Colour {
        add_colours(&self, rhs)
    }
}

impl ops::Add<Colour> for Colour {
    type Output = Colour;

    fn add(self, rhs: Colour) -> Colour {
        add_colours(&self, &rhs)
    }
}

impl ops::Mul<&Colour> for &Colour {
    type Output = Colour;

    fn mul(self, rhs: &Colour) -> Colour {
        mul_colours(self, rhs)
    }
}

impl ops::Mul<Colour> for &Colour {
    type Output = Colour;

    fn mul(self, rhs: Colour) -> Colour {
        mul_colours(self, &rhs)
    }
}

impl ops::Mul<&Colour> for Colour {
    type Output = Colour;

    fn mul(self, rhs: &Colour) -> Colour {
        mul_colours(&self, rhs)
    }
}

impl ops::Mul<Colour> for Colour {
    type Output = Colour;

    fn mul(self, rhs: Colour) -> Colour {
        mul_colours(&self, &rhs)
    }
}

impl ops::Add<f64> for &Colour {
    type Output = Colour;

    fn add(self, rhs: f64) -> Colour {
        add_colour_and_scalar(self, rhs)
    }
}

impl ops::Add<&Colour> for f64 {
    type Output = Colour;

    fn add(self, rhs: &Colour) -> Colour {
        add_colour_and_scalar(rhs, self)
    }
}

impl ops::Add<f64> for Colour {
    type Output = Colour;

    fn add(self, rhs: f64) -> Colour {
        add_colour_and_scalar(&self, rhs)
    }
}

impl ops::Add<Colour> for f64 {
    type Output = Colour;

    fn add(self, rhs: Colour) -> Colour {
        add_colour_and_scalar(&rhs, self)
    }
}

impl ops::Mul<f64> for &Colour {
    type Output = Colour;

    fn mul(self, rhs: f64) -> Colour {
        mul_colour_and_scalar(self, rhs)
    }
}

impl ops::Mul<f64> for Colour {
    type Output = Colour;

    fn mul(self, rhs: f64) -> Colour {
        mul_colour_and_scalar(&self, rhs)
    }
}

impl ops::Mul<&Colour> for f64 {
    type Output = Colour;

    fn mul(self, rhs: &Colour) -> Colour {
        mul_colour_and_scalar(rhs, self)
    }
}

impl ops::Mul<Colour> for f64 {
    type Output = Colour;

    fn mul(self, rhs: Colour) -> Colour {
        mul_colour_and_scalar(&rhs, self)
    }
}

impl ops::Div<f64> for &Colour {
    type Output = Colour;

    fn div(self, rhs: f64) -> Colour {
        div_colour_and_scalar(self, rhs)
    }
}

impl ops::Div<f64> for Colour {
    type Output = Colour;

    fn div(self, rhs: f64) -> Colour {
        div_colour_and_scalar(&self, rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_colour_into_rgb() {
        let colour = Colour {
            r: 1.0,
            g: 0.5,
            b: 0.0,
        };

        let rgb = colour.to_rgb();

        assert_eq!(rgb[0], 255);
        assert_eq!(rgb[1], 127);
        assert_eq!(rgb[2], 0);
    }

    #[test]
    fn test_colour_len() {
        let colour = Colour::new(1.0, 2.0, 3.0);

        let expected_result = 3.741_657_386_773_941_3;

        assert_approx_eq!(colour.len(), expected_result);
    }

    #[test]
    fn test_colour_unit_vector() {
        let colour = Colour::new(1.0, 2.0, 3.0);

        assert_approx_eq!(colour.unit_vector().len(), 1.0)
    }

    #[test]
    fn test_sum_colours() {
        let colours = vec![
            Colour {
                r: 1.0,
                g: 0.5,
                b: 0.0,
            },
            Colour {
                r: 1.0,
                g: 0.5,
                b: 0.0,
            },
        ];

        let expected_result = Colour {
            r: 2.0,
            g: 1.0,
            b: 0.0,
        };
        let actual_result: Colour = colours.iter().cloned().sum();

        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn test_add_colours() {
        let colour_a = Colour {
            r: 1.0,
            g: 1.5,
            b: 2.0,
        };
        let colour_b = Colour {
            r: -1.0,
            g: 0.5,
            b: 0.0,
        };

        let expected_result = Colour {
            r: 0.0,
            g: 2.0,
            b: 2.0,
        };

        assert_eq!(colour_a + colour_b, expected_result);
        assert_eq!(colour_a + colour_b, expected_result);
        assert_eq!(colour_a + colour_b, expected_result);
        assert_eq!(colour_a + colour_b, expected_result);
    }

    #[test]
    fn test_mul_colours() {
        let colour_a = Colour {
            r: 1.0,
            g: 1.5,
            b: 2.0,
        };
        let colour_b = Colour {
            r: -1.0,
            g: 0.5,
            b: 0.0,
        };

        let expected_result = Colour {
            r: -1.0,
            g: 0.75,
            b: 0.0,
        };

        assert_eq!(colour_a * colour_b, expected_result);
        assert_eq!(colour_a * colour_b, expected_result);
        assert_eq!(colour_a * colour_b, expected_result);
        assert_eq!(colour_a * colour_b, expected_result);
    }

    #[test]
    fn test_add_colour_and_scalar() {
        let colour = Colour {
            r: 1.0,
            g: 1.5,
            b: 2.0,
        };
        let scalar = 2.0;

        let expected_result = Colour {
            r: 3.0,
            g: 3.5,
            b: 4.0,
        };

        assert_eq!(colour + scalar, expected_result);
        assert_eq!(colour + scalar, expected_result);
        assert_eq!(scalar + colour, expected_result);
        assert_eq!(scalar + colour, expected_result);
    }

    #[test]
    fn test_mul_colour_and_scalar() {
        let colour = Colour {
            r: 1.0,
            g: 1.5,
            b: 2.0,
        };
        let scalar = 2.0;

        let expected_result = Colour {
            r: 2.0,
            g: 3.0,
            b: 4.0,
        };

        assert_eq!(colour * scalar, expected_result);
        assert_eq!(colour * scalar, expected_result);
        assert_eq!(scalar * colour, expected_result);
        assert_eq!(scalar * colour, expected_result);
    }

    #[test]
    fn test_div_colour_and_scalar() {
        let colour = Colour {
            r: 1.0,
            g: 1.5,
            b: 2.0,
        };
        let scalar = 2.0;

        let expected_result = Colour {
            r: 0.5,
            g: 0.75,
            b: 1.0,
        };

        assert_eq!(colour / scalar, expected_result);
        assert_eq!(colour / scalar, expected_result);
    }
}
