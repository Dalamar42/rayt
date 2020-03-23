use float;
use std::f64::consts::PI;
use std::ops;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector { x, y, z }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn len(&self) -> f64 {
        (&self).len_squared().sqrt()
    }

    pub fn len_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn unit_vector(&self) -> Vector {
        let k = 1.0 / self.len();
        Vector {
            x: self.x * k,
            y: self.y * k,
            z: self.z * k,
        }
    }

    pub fn dot(lhs: &Vector, rhs: &Vector) -> f64 {
        lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z
    }

    pub fn cross(lhs: &Vector, rhs: &Vector) -> Vector {
        Vector {
            x: lhs.y * rhs.z - lhs.z * rhs.y,
            y: -(lhs.x * rhs.z - lhs.z * rhs.x),
            z: lhs.x * rhs.y - lhs.y * rhs.x,
        }
    }

    pub fn min(&self, vector: &Vector) -> Vector {
        Vector {
            x: float::min(vector.x(), self.x()),
            y: float::min(vector.y(), self.y()),
            z: float::min(vector.z(), self.z()),
        }
    }

    pub fn max(&self, vector: &Vector) -> Vector {
        Vector {
            x: float::max(vector.x(), self.x()),
            y: float::max(vector.y(), self.y()),
            z: float::max(vector.z(), self.z()),
        }
    }

    pub fn rotate_y(&self, angle: f64) -> Vector {
        let radians = (PI / 180.0) * angle;
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        Vector {
            x: cos_theta * self.x + sin_theta * self.z,
            y: self.y,
            z: -sin_theta * self.x + cos_theta * self.z,
        }
    }
}

fn add_vectors(lhs: &Vector, rhs: &Vector) -> Vector {
    Vector {
        x: lhs.x + rhs.x,
        y: lhs.y + rhs.y,
        z: lhs.z + rhs.z,
    }
}

fn subtract_vectors(lhs: &Vector, rhs: &Vector) -> Vector {
    Vector {
        x: lhs.x - rhs.x,
        y: lhs.y - rhs.y,
        z: lhs.z - rhs.z,
    }
}

fn neg_vector(vector: &Vector) -> Vector {
    Vector {
        x: -vector.x,
        y: -vector.y,
        z: -vector.z,
    }
}

fn add_vector_and_scalar(lhs: &Vector, rhs: f64) -> Vector {
    Vector {
        x: lhs.x + rhs,
        y: lhs.y + rhs,
        z: lhs.z + rhs,
    }
}

fn mul_vector_and_scalar(lhs: &Vector, rhs: f64) -> Vector {
    Vector {
        x: lhs.x * rhs,
        y: lhs.y * rhs,
        z: lhs.z * rhs,
    }
}

fn div_vector_and_scalar(lhs: &Vector, rhs: f64) -> Vector {
    Vector {
        x: lhs.x / rhs,
        y: lhs.y / rhs,
        z: lhs.z / rhs,
    }
}

impl ops::Add<&Vector> for &Vector {
    type Output = Vector;

    fn add(self, rhs: &Vector) -> Vector {
        add_vectors(self, rhs)
    }
}

impl ops::Add<Vector> for &Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Vector {
        add_vectors(self, &rhs)
    }
}

impl ops::Add<&Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: &Vector) -> Vector {
        add_vectors(&self, rhs)
    }
}

impl ops::Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Vector {
        add_vectors(&self, &rhs)
    }
}

impl ops::Sub<&Vector> for &Vector {
    type Output = Vector;

    fn sub(self, rhs: &Vector) -> Vector {
        subtract_vectors(self, rhs)
    }
}

impl ops::Sub<Vector> for &Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Vector {
        subtract_vectors(self, &rhs)
    }
}

impl ops::Sub<&Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: &Vector) -> Vector {
        subtract_vectors(&self, rhs)
    }
}

impl ops::Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Vector {
        subtract_vectors(&self, &rhs)
    }
}

impl ops::Neg for &Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        neg_vector(&self)
    }
}

impl ops::Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        neg_vector(&self)
    }
}

impl ops::Add<f64> for &Vector {
    type Output = Vector;

    fn add(self, rhs: f64) -> Vector {
        add_vector_and_scalar(self, rhs)
    }
}

impl ops::Add<&Vector> for f64 {
    type Output = Vector;

    fn add(self, rhs: &Vector) -> Vector {
        add_vector_and_scalar(rhs, self)
    }
}

impl ops::Add<f64> for Vector {
    type Output = Vector;

    fn add(self, rhs: f64) -> Vector {
        add_vector_and_scalar(&self, rhs)
    }
}

impl ops::Add<Vector> for f64 {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Vector {
        add_vector_and_scalar(&rhs, self)
    }
}

impl ops::Mul<f64> for &Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Vector {
        mul_vector_and_scalar(self, rhs)
    }
}

impl ops::Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Vector {
        mul_vector_and_scalar(&self, rhs)
    }
}

impl ops::Mul<&Vector> for f64 {
    type Output = Vector;

    fn mul(self, rhs: &Vector) -> Vector {
        mul_vector_and_scalar(rhs, self)
    }
}

impl ops::Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Vector {
        mul_vector_and_scalar(&rhs, self)
    }
}

impl ops::Div<f64> for Vector {
    type Output = Vector;

    fn div(self, rhs: f64) -> Vector {
        div_vector_and_scalar(&self, rhs)
    }
}

impl ops::Div<f64> for &Vector {
    type Output = Vector;

    fn div(self, rhs: f64) -> Vector {
        div_vector_and_scalar(&self, rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_vector_len() {
        let vector = Vector {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };

        let expected_result = 3.741_657_386_773_941_3;

        assert_approx_eq!(vector.len(), expected_result);
    }

    #[test]
    fn test_vector_unit_vector() {
        let vector = Vector {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };

        assert_approx_eq!(vector.unit_vector().len(), 1.0)
    }

    #[test]
    fn test_add_vectors() {
        let vector_a = Vector {
            x: 1.0,
            y: 1.5,
            z: 2.0,
        };
        let vector_b = Vector {
            x: -1.0,
            y: 0.5,
            z: 0.0,
        };

        let expected_result = Vector {
            x: 0.0,
            y: 2.0,
            z: 2.0,
        };

        assert_eq!(vector_a + vector_b, expected_result);
        assert_eq!(vector_a + vector_b, expected_result);
        assert_eq!(vector_a + vector_b, expected_result);
        assert_eq!(vector_a + vector_b, expected_result);
    }

    #[test]
    fn test_subtract_vectors() {
        let vector_a = Vector {
            x: 1.0,
            y: 1.5,
            z: 2.0,
        };
        let vector_b = Vector {
            x: -1.0,
            y: 0.5,
            z: 0.0,
        };

        let expected_result = Vector {
            x: 2.0,
            y: 1.0,
            z: 2.0,
        };

        assert_eq!(vector_a - vector_b, expected_result);
        assert_eq!(vector_a - vector_b, expected_result);
        assert_eq!(vector_a - vector_b, expected_result);
        assert_eq!(vector_a - vector_b, expected_result);
    }

    #[test]
    fn test_neg_vector() {
        let vector = Vector {
            x: 1.0,
            y: 1.5,
            z: 2.0,
        };

        let expected_result = Vector {
            x: -1.0,
            y: -1.5,
            z: -2.0,
        };

        assert_eq!(-vector, expected_result);
        assert_eq!(-&vector, expected_result);
    }

    #[test]
    fn test_add_vector_and_scalar() {
        let vector = Vector {
            x: 1.0,
            y: 1.5,
            z: 2.0,
        };
        let scalar = 2.0;

        let expected_result = Vector {
            x: 3.0,
            y: 3.5,
            z: 4.0,
        };

        assert_eq!(vector + scalar, expected_result);
        assert_eq!(vector + scalar, expected_result);
        assert_eq!(scalar + vector, expected_result);
        assert_eq!(scalar + vector, expected_result);
    }

    #[test]
    fn test_mul_vector_and_scalar() {
        let vector = Vector {
            x: 1.0,
            y: 1.5,
            z: 2.0,
        };
        let scalar = 2.0;

        let expected_result = Vector {
            x: 2.0,
            y: 3.0,
            z: 4.0,
        };

        assert_eq!(vector * scalar, expected_result);
        assert_eq!(vector * scalar, expected_result);
        assert_eq!(scalar * vector, expected_result);
        assert_eq!(scalar * vector, expected_result);
    }

    #[test]
    fn test_div_vector_and_scalar() {
        let vector = Vector {
            x: 1.0,
            y: 1.5,
            z: 2.0,
        };
        let scalar = 2.0;

        let expected_result = Vector {
            x: 0.5,
            y: 0.75,
            z: 1.0,
        };

        assert_eq!(vector / scalar, expected_result);
        assert_eq!(vector / scalar, expected_result);
    }

    #[test]
    fn test_vectors_dot_product() {
        let vector_a = Vector {
            x: 1.0,
            y: 1.5,
            z: 2.0,
        };
        let vector_b = Vector {
            x: -1.0,
            y: 0.5,
            z: 0.0,
        };

        let expected_result = -0.25;

        assert_approx_eq!(Vector::dot(&vector_a, &vector_b), expected_result);
    }

    #[test]
    fn test_vectors_cross_product() {
        let vector_a = Vector {
            x: 2.0,
            y: 3.0,
            z: 4.0,
        };
        let vector_b = Vector {
            x: 5.0,
            y: 6.0,
            z: 7.0,
        };

        let expected_result = Vector {
            x: -3.0,
            y: 6.0,
            z: -3.0,
        };

        assert_eq!(Vector::cross(&vector_a, &vector_b), expected_result);
    }

    #[test]
    fn test_vector_min() {
        let vector_a = Vector::new(1.0, 2.0, 3.0);
        let vector_b = Vector::new(1.0, 1.0, 4.0);

        let expected_result = Vector::new(1.0, 1.0, 3.0);

        assert_eq!(vector_a.min(&vector_b), expected_result);
    }

    #[test]
    fn test_vector_max() {
        let vector_a = Vector::new(1.0, 2.0, 3.0);
        let vector_b = Vector::new(1.0, 1.0, 4.0);

        let expected_result = Vector::new(1.0, 2.0, 4.0);

        assert_eq!(vector_a.max(&vector_b), expected_result);
    }

    #[test]
    fn test_rotate_y() {
        let vector = Vector::new(3.0, 0.5, 0.5);

        let rotated = vector.rotate_y(90.0);
        assert_approx_eq!(rotated.x(), 0.5);
        assert_approx_eq!(rotated.y(), 0.5);
        assert_approx_eq!(rotated.z(), -3.0);

        let rotated = vector.rotate_y(-90.0);
        assert_approx_eq!(rotated.x(), -0.5);
        assert_approx_eq!(rotated.y(), 0.5);
        assert_approx_eq!(rotated.z(), 3.0);
    }
}
