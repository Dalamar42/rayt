use std::ops;
use std::iter::Sum;

#[derive(Debug, Clone, PartialEq)]
pub struct Colour {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Colour {
    pub fn r_norm(&self) -> u8 {
        assert!(0.0 <= self.r && self.r <= 1.0);
        return (255.99 * &self.r) as u8
    }

    pub fn g_norm(&self) -> u8 {
        assert!(0.0 <= self.g && self.g <= 1.0);
        return (255.99 * &self.g) as u8
    }

    pub fn b_norm(&self) -> u8 {
        assert!(0.0 <= self.b && self.b <= 1.0);
        return (255.99 * &self.b) as u8
    }
}

impl Sum<Colour> for Colour {
    fn sum<I: Iterator<Item=Colour>>(iter: I) -> Colour {
        let mut sum = Colour {r: 0.0, g: 0.0, b: 0.0};
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

    #[test]
    fn test_normalise_colour() {
        let colour = Colour {r: 1.0, g: 0.5, b: 0.0};

        assert_eq!(colour.r_norm(), 255);
        assert_eq!(colour.g_norm(), 127);
        assert_eq!(colour.b_norm(), 0);
    }

    #[test]
    fn test_sum_colours() {
        let colours = vec![
            Colour {r: 1.0, g: 0.5, b: 0.0},
            Colour {r: 1.0, g: 0.5, b: 0.0},
        ];

        let expected_result = Colour {r: 2.0, g: 1.0, b: 0.0};
        let actual_result: Colour = colours.iter().cloned().sum();

        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn test_add_colours() {
        let colour_a = Colour {r: 1.0, g: 1.5, b: 2.0};
        let colour_b = Colour {r: -1.0, g: 0.5, b: 0.0};

        let expected_result = Colour {r: 0.0, g: 2.0, b: 2.0};

        assert_eq!(colour_a.clone() + colour_b.clone(), expected_result);
        assert_eq!(&colour_a + colour_b.clone(), expected_result);
        assert_eq!(colour_a.clone() + &colour_b, expected_result);
        assert_eq!(&colour_a + &colour_b, expected_result);
    }

    #[test]
    fn test_add_colour_and_scalar() {
        let colour = Colour {r: 1.0, g: 1.5, b: 2.0};
        let scalar = 2.0;

        let expected_result = Colour {r: 3.0, g: 3.5, b: 4.0};

        assert_eq!(colour.clone() + scalar, expected_result);
        assert_eq!(&colour + scalar, expected_result);
        assert_eq!(scalar + colour.clone(), expected_result);
        assert_eq!(scalar + &colour, expected_result);
    }

    #[test]
    fn test_mul_colour_and_scalar() {
        let colour = Colour {r: 1.0, g: 1.5, b: 2.0};
        let scalar = 2.0;

        let expected_result = Colour {r: 2.0, g: 3.0, b: 4.0};

        assert_eq!(colour.clone() * scalar, expected_result);
        assert_eq!(&colour * scalar, expected_result);
        assert_eq!(scalar * colour.clone(), expected_result);
        assert_eq!(scalar * &colour, expected_result);
    }

    #[test]
    fn test_div_colour_and_scalar() {
        let colour = Colour {r: 1.0, g: 1.5, b: 2.0};
        let scalar = 2.0;

        let expected_result = Colour {r: 0.5, g: 0.75, b: 1.0};

        assert_eq!(colour.clone() / scalar, expected_result);
        assert_eq!(&colour / scalar, expected_result);
    }
}
