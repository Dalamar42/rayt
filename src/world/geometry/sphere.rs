use camera::Ray;
use data::vector::Vector;
use world::geometry::Geometry;

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
    pub centre: Vector,
    pub radius: f64,
}

#[typetag::serde]
impl Geometry for Sphere {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<f64> {
        sphere_hit(ray, &self.centre, self.radius, tmin, tmax)
    }

    fn surface_normal(&self, ray: &Ray, distance: f64) -> Vector {
        // We divide by radius instead of taking the unit vector so that a negative
        // radius sphere will have a surface normal that points inward
        (ray.point(distance) - &self.centre) / self.radius
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MovingSphere {
    pub centre_start: Vector,
    pub time_start: f64,
    pub centre_end: Vector,
    pub time_end: f64,
    pub radius: f64,
}

impl MovingSphere {
    fn centre(&self, time: f64) -> Vector {
        let time_fraction = (time - self.time_start) / (self.time_end - self.time_start);
        &self.centre_start + time_fraction * (&self.centre_end - &self.centre_start)
    }
}

#[typetag::serde]
impl Geometry for MovingSphere {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<f64> {
        let centre = self.centre(ray.time());
        sphere_hit(ray, &centre, self.radius, tmin, tmax)
    }

    fn surface_normal(&self, ray: &Ray, distance: f64) -> Vector {
        // We divide by radius instead of taking the unit vector so that a negative
        // radius sphere will have a surface normal that points inward
        let centre = self.centre(ray.time());
        (ray.point(distance) - &centre) / self.radius
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_sphere_hit() {
        let sphere = Sphere {
            centre: Vector {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            radius: 1.0,
        };
        let ray = Ray::new(
            Vector {
                x: -2.0,
                y: 0.0,
                z: 0.0,
            },
            Vector {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            0.0,
        );

        let hit_distance = sphere.hit(&ray, 0.0, core::f64::MAX).unwrap();

        assert_approx_eq!(hit_distance, 1.0);
    }

    #[test]
    fn test_neg_radius_sphere_hit() {
        let sphere = Sphere {
            centre: Vector {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            radius: -1.0,
        };
        let ray = Ray::new(
            Vector {
                x: -2.0,
                y: 0.0,
                z: 0.0,
            },
            Vector {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            0.0,
        );

        let hit_distance = sphere.hit(&ray, 0.0, core::f64::MAX).unwrap();

        assert_approx_eq!(hit_distance, 1.0);
    }

    #[test]
    fn test_sphere_surface_normal() {
        let sphere = Sphere {
            centre: Vector {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            radius: 1.0,
        };
        let ray = Ray::new(
            Vector {
                x: -2.0,
                y: 0.0,
                z: 0.0,
            },
            Vector {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            0.0,
        );

        let hit_distance = sphere.hit(&ray, 0.0, core::f64::MAX).unwrap();
        let surface_normal = sphere.surface_normal(&ray, hit_distance);

        assert_eq!(
            surface_normal,
            Vector {
                x: -1.0,
                y: 0.0,
                z: 0.0
            }
        );
    }

    #[test]
    fn test_neg_radius_sphere_surface_normal() {
        let sphere = Sphere {
            centre: Vector {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            radius: -1.0,
        };
        let ray = Ray::new(
            Vector {
                x: -2.0,
                y: 0.0,
                z: 0.0,
            },
            Vector {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            0.0,
        );

        let hit_distance = sphere.hit(&ray, 0.0, core::f64::MAX).unwrap();
        let surface_normal = sphere.surface_normal(&ray, hit_distance);

        assert_eq!(
            surface_normal,
            Vector {
                x: 1.0,
                y: 0.0,
                z: 0.0
            }
        );
    }
}
