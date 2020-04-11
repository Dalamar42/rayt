use crate::camera::Ray;
use crate::data::vector::Vector;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AxisAlignedBoundingBox {
    min: Vector,
    max: Vector,
}

fn single_axis_hit(min: f64, max: f64, origin: f64, direction: f64) -> (f64, f64) {
    let inv_direction = 1.0 / direction;

    let t0 = (min - origin) * inv_direction;
    let t1 = (max - origin) * inv_direction;

    if inv_direction < 0.0 {
        (t1, t0)
    } else {
        (t0, t1)
    }
}

fn surrounding(
    box_a: &AxisAlignedBoundingBox,
    box_b: &AxisAlignedBoundingBox,
) -> AxisAlignedBoundingBox {
    let small = Vector::new(
        f64::min(box_a.min().x(), box_b.min().x()),
        f64::min(box_a.min().y(), box_b.min().y()),
        f64::min(box_a.min().z(), box_b.min().z()),
    );
    let big = Vector::new(
        f64::max(box_a.max().x(), box_b.max().x()),
        f64::max(box_a.max().y(), box_b.max().y()),
        f64::max(box_a.max().z(), box_b.max().z()),
    );

    AxisAlignedBoundingBox {
        min: small,
        max: big,
    }
}

impl AxisAlignedBoundingBox {
    pub fn new(a: Vector, b: Vector) -> AxisAlignedBoundingBox {
        AxisAlignedBoundingBox { min: a, max: b }
    }

    pub fn surrounding(
        box_a: &Option<AxisAlignedBoundingBox>,
        box_b: &Option<AxisAlignedBoundingBox>,
    ) -> Option<AxisAlignedBoundingBox> {
        let box_a = match box_a {
            Some(bounding_box) => bounding_box,
            None => return None,
        };
        let box_b = match box_b {
            Some(bounding_box) => bounding_box,
            None => return None,
        };

        Some(surrounding(&box_a, &box_b))
    }

    pub fn min(&self) -> &Vector {
        &self.min
    }

    pub fn max(&self) -> &Vector {
        &self.max
    }

    pub fn intersection(&self, ray: &Ray, tmin: f64, tmax: f64) -> bool {
        let (t0, t1) = single_axis_hit(
            self.min.x(),
            self.max.x(),
            ray.origin().x(),
            ray.direction().x(),
        );
        let tmin = if t0 > tmin { t0 } else { tmin };
        let tmax = if t1 < tmax { t1 } else { tmax };
        if tmax <= tmin {
            return false;
        }

        let (t0, t1) = single_axis_hit(
            self.min.y(),
            self.max.y(),
            ray.origin().y(),
            ray.direction().y(),
        );
        let tmin = if t0 > tmin { t0 } else { tmin };
        let tmax = if t1 < tmax { t1 } else { tmax };
        if tmax <= tmin {
            return false;
        }

        let (t0, t1) = single_axis_hit(
            self.min.z(),
            self.max.z(),
            ray.origin().z(),
            ray.direction().z(),
        );
        let tmin = if t0 > tmin { t0 } else { tmin };
        let tmax = if t1 < tmax { t1 } else { tmax };
        if tmax <= tmin {
            return false;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_surrounding_bounding_boxes() {
        let box_a =
            AxisAlignedBoundingBox::new(Vector::new(0.0, 0.0, 0.0), Vector::new(2.0, 2.0, 2.0));
        let box_b =
            AxisAlignedBoundingBox::new(Vector::new(0.5, 0.5, 0.5), Vector::new(4.0, 2.0, 1.0));

        let expected_box =
            AxisAlignedBoundingBox::new(Vector::new(0.0, 0.0, 0.0), Vector::new(4.0, 2.0, 2.0));

        assert_eq!(
            Some(expected_box),
            AxisAlignedBoundingBox::surrounding(&Some(box_a), &Some(box_b)),
        );
    }

    #[test]
    fn test_surrounding_bounding_boxes_when_one_is_none() {
        let bounding_box =
            AxisAlignedBoundingBox::new(Vector::new(0.0, 0.0, 0.0), Vector::new(2.0, 2.0, 2.0));

        assert_eq!(
            None,
            AxisAlignedBoundingBox::surrounding(&Some(bounding_box), &None),
        );
    }
}
