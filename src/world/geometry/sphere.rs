use camera::Ray;
use data::vector::Vector;
use world::geometry::{Geometry, HitResult};

fn sphere_hit(ray: &Ray, centre: &Vector, radius: f64, tmin: f64, tmax: f64) -> Option<f64> {
    // p(t) = ray
    // c = sphere_centre
    // R = sphere_radius
    // dot((p(t) - c), (p(t) - c)) = R^2

    let oc = ray.origin() - centre;

    let a = Vector::dot(&ray.direction(), &ray.direction());
    let b = 2.0 * Vector::dot(&oc, &ray.direction());
    let c = Vector::dot(&oc, &oc) - radius * radius;

    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        return Option::None;
    }

    let t = (-b - discriminant.sqrt()) / (2.0 * a);
    if t < tmin || t > tmax {
        return Option::None;
    }

    Option::Some(t)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Sphere {
    centre: Vector,
    radius: f64,
}

impl Sphere {
    pub fn new(centre: Vector, radius: f64) -> Sphere {
        Sphere { centre, radius }
    }

    fn surface_normal(&self, ray: &Ray, distance: f64) -> Vector {
        // We divide by radius instead of taking the unit vector so that a negative
        // radius sphere will have a surface normal that points inward
        (ray.point(distance) - &self.centre) / self.radius
    }
}

#[typetag::serde]
impl Geometry for Sphere {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> HitResult {
        let hit_distance = sphere_hit(ray, &self.centre, self.radius, tmin, tmax);
        if hit_distance.is_none() {
            return HitResult::Miss;
        }

        let distance = hit_distance.unwrap();

        let point = ray.point(distance);
        let surface_normal = self.surface_normal(&ray, distance);

        HitResult::Hit {
            distance,
            ray: ray.clone(),
            point,
            surface_normal,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MovingSphere {
    centre_start: Vector,
    time_start: f64,
    centre_end: Vector,
    time_end: f64,
    radius: f64,
}

impl MovingSphere {
    pub fn new(
        centre_start: Vector,
        time_start: f64,
        centre_end: Vector,
        time_end: f64,
        radius: f64,
    ) -> MovingSphere {
        MovingSphere {
            centre_start,
            time_start,
            centre_end,
            time_end,
            radius,
        }
    }

    fn centre(&self, time: f64) -> Vector {
        let time_fraction = (time - self.time_start) / (self.time_end - self.time_start);
        &self.centre_start + time_fraction * (&self.centre_end - &self.centre_start)
    }

    fn surface_normal(&self, ray: &Ray, distance: f64) -> Vector {
        // We divide by radius instead of taking the unit vector so that a negative
        // radius sphere will have a surface normal that points inward
        let centre = self.centre(ray.time());
        (ray.point(distance) - &centre) / self.radius
    }
}

#[typetag::serde]
impl Geometry for MovingSphere {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> HitResult {
        let centre = self.centre(ray.time());
        let hit_distance = sphere_hit(ray, &centre, self.radius, tmin, tmax);
        if hit_distance.is_none() {
            return HitResult::Miss;
        }

        let distance = hit_distance.unwrap();

        let point = ray.point(distance);
        let surface_normal = self.surface_normal(&ray, distance);

        HitResult::Hit {
            distance,
            ray: ray.clone(),
            point,
            surface_normal,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_sphere_hit() {
        let sphere = Sphere {
            centre: Vector::new(0.0, 0.0, 0.0),
            radius: 1.0,
        };
        let ray = Ray::new(Vector::new(-2.0, 0.0, 0.0), Vector::new(1.0, 0.0, 0.0), 0.0);

        let hit_result = sphere.hit(&ray, 0.0, core::f64::MAX);

        match hit_result {
            HitResult::Hit {
                distance,
                point: _,
                ray: _,
                surface_normal: _,
            } => {
                assert_approx_eq!(distance, 1.0);
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn test_neg_radius_sphere_hit() {
        let sphere = Sphere {
            centre: Vector::new(0.0, 0.0, 0.0),
            radius: -1.0,
        };
        let ray = Ray::new(Vector::new(-2.0, 0.0, 0.0), Vector::new(1.0, 0.0, 0.0), 0.0);

        let hit_result = sphere.hit(&ray, 0.0, core::f64::MAX);

        match hit_result {
            HitResult::Hit {
                distance,
                point: _,
                ray: _,
                surface_normal: _,
            } => {
                assert_approx_eq!(distance, 1.0);
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn test_sphere_surface_normal() {
        let sphere = Sphere {
            centre: Vector::new(0.0, 0.0, 0.0),
            radius: 1.0,
        };
        let ray = Ray::new(Vector::new(-2.0, 0.0, 0.0), Vector::new(1.0, 0.0, 0.0), 0.0);

        let hit_result = sphere.hit(&ray, 0.0, core::f64::MAX);

        match hit_result {
            HitResult::Hit {
                distance: _,
                point: _,
                ray: _,
                surface_normal,
            } => {
                assert_eq!(surface_normal, Vector::new(-1.0, 0.0, 0.0));
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn test_neg_radius_sphere_surface_normal() {
        let sphere = Sphere {
            centre: Vector::new(0.0, 0.0, 0.0),
            radius: -1.0,
        };
        let ray = Ray::new(Vector::new(-2.0, 0.0, 0.0), Vector::new(1.0, 0.0, 0.0), 0.0);

        let hit_result = sphere.hit(&ray, 0.0, core::f64::MAX);

        match hit_result {
            HitResult::Hit {
                distance: _,
                point: _,
                ray: _,
                surface_normal,
            } => {
                assert_eq!(surface_normal, Vector::new(1.0, 0.0, 0.0));
            }
            _ => assert!(false),
        }
    }
}
