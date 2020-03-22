use camera::Ray;
use data::assets::Assets;
use data::vector::Vector;
use failure::Error;
use std::f64::consts::PI;
use world::geometry::axis_aligned_bounding_box::AxisAlignedBoundingBox;
use world::geometry::{Geometry, HitResult};
use world::materials::Material;

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

fn sphere_bounding_box(centre: &Vector, radius: f64) -> Option<AxisAlignedBoundingBox> {
    let radius = radius.abs();
    Some(AxisAlignedBoundingBox::new(
        centre - Vector::new(radius, radius, radius),
        centre + Vector::new(radius, radius, radius),
    ))
}

fn sphere_texture_coords(hit_point: &Vector, centre: &Vector, radius: f64) -> (f64, f64) {
    let point = (hit_point - centre) / radius;

    let theta = PI - f64::acos(point.y());
    let phi = f64::atan2(point.x(), point.z());

    let row = theta / PI;
    let mut col = phi / (2.0 * PI) + 0.25;

    // Rotate frame so the middle of the image texture is facing the camera
    col += 0.25;
    if col > 1.0 {
        col -= 1.0;
    }

    (row, col)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Sphere {
    centre: Vector,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub fn new(centre: Vector, radius: f64, material: Material) -> Sphere {
        Sphere {
            centre,
            radius,
            material,
        }
    }

    fn surface_normal(&self, ray: &Ray, distance: f64) -> Vector {
        // We divide by radius instead of taking the unit vector so that a negative
        // radius sphere will have a surface normal that points inward
        (ray.point(distance) - self.centre) / self.radius
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

        let texture_coords = sphere_texture_coords(&point, &self.centre, self.radius);

        HitResult::Hit {
            distance,
            ray: ray.clone(),
            point,
            surface_normal,
            material: self.material.clone(),
            texture_coords,
        }
    }

    fn bounding_box(&self, _time_start: f64, _time_end: f64) -> Option<AxisAlignedBoundingBox> {
        sphere_bounding_box(&self.centre, self.radius)
    }

    fn validate(&self, assets: &Assets) -> Result<(), Error> {
        Ok(self.material.validate(assets)?)
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MovingSphere {
    centre_start: Vector,
    time_start: f64,
    centre_end: Vector,
    time_end: f64,
    radius: f64,
    material: Material,
}

impl MovingSphere {
    pub fn new(
        centre_start: Vector,
        time_start: f64,
        centre_end: Vector,
        time_end: f64,
        radius: f64,
        material: Material,
    ) -> MovingSphere {
        MovingSphere {
            centre_start,
            time_start,
            centre_end,
            time_end,
            radius,
            material,
        }
    }

    fn centre(&self, time: f64) -> Vector {
        let time_fraction = (time - self.time_start) / (self.time_end - self.time_start);
        self.centre_start + time_fraction * (self.centre_end - self.centre_start)
    }

    fn surface_normal(&self, ray: &Ray, distance: f64) -> Vector {
        // We divide by radius instead of taking the unit vector so that a negative
        // radius sphere will have a surface normal that points inward
        let centre = self.centre(ray.time());
        (ray.point(distance) - centre) / self.radius
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

        let texture_coords = sphere_texture_coords(&point, &centre, self.radius);

        HitResult::Hit {
            distance,
            ray: ray.clone(),
            point,
            surface_normal,
            material: self.material.clone(),
            texture_coords,
        }
    }

    fn bounding_box(&self, time_start: f64, time_end: f64) -> Option<AxisAlignedBoundingBox> {
        let box_start = sphere_bounding_box(&self.centre(time_start), self.radius);
        let box_end = sphere_bounding_box(&self.centre(time_end), self.radius);

        AxisAlignedBoundingBox::surrounding(&box_start, &box_end)
    }

    fn validate(&self, assets: &Assets) -> Result<(), Error> {
        self.material.validate(assets)
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
            material: Material::Dielectric {
                refractive_index: 1.5,
            },
        };
        let ray = Ray::new(Vector::new(-2.0, 0.0, 0.0), Vector::new(1.0, 0.0, 0.0), 0.0);

        let hit_result = sphere.hit(&ray, 0.0, core::f64::MAX);

        match hit_result {
            HitResult::Hit { distance, .. } => {
                assert_approx_eq!(distance, 1.0);
            }
            _ => panic!(),
        }
    }

    #[test]
    fn test_neg_radius_sphere_hit() {
        let sphere = Sphere {
            centre: Vector::new(0.0, 0.0, 0.0),
            radius: -1.0,
            material: Material::Dielectric {
                refractive_index: 1.5,
            },
        };
        let ray = Ray::new(Vector::new(-2.0, 0.0, 0.0), Vector::new(1.0, 0.0, 0.0), 0.0);

        let hit_result = sphere.hit(&ray, 0.0, core::f64::MAX);

        match hit_result {
            HitResult::Hit { distance, .. } => {
                assert_approx_eq!(distance, 1.0);
            }
            _ => panic!(),
        }
    }

    #[test]
    fn test_sphere_surface_normal() {
        let sphere = Sphere {
            centre: Vector::new(0.0, 0.0, 0.0),
            radius: 1.0,
            material: Material::Dielectric {
                refractive_index: 1.5,
            },
        };
        let ray = Ray::new(Vector::new(-2.0, 0.0, 0.0), Vector::new(1.0, 0.0, 0.0), 0.0);

        let hit_result = sphere.hit(&ray, 0.0, core::f64::MAX);

        match hit_result {
            HitResult::Hit { surface_normal, .. } => {
                assert_eq!(surface_normal, Vector::new(-1.0, 0.0, 0.0));
            }
            _ => panic!(),
        }
    }

    #[test]
    fn test_neg_radius_sphere_surface_normal() {
        let sphere = Sphere {
            centre: Vector::new(0.0, 0.0, 0.0),
            radius: -1.0,
            material: Material::Dielectric {
                refractive_index: 1.5,
            },
        };
        let ray = Ray::new(Vector::new(-2.0, 0.0, 0.0), Vector::new(1.0, 0.0, 0.0), 0.0);

        let hit_result = sphere.hit(&ray, 0.0, core::f64::MAX);

        match hit_result {
            HitResult::Hit { surface_normal, .. } => {
                assert_eq!(surface_normal, Vector::new(1.0, 0.0, 0.0));
            }
            _ => panic!(),
        }
    }

    #[test]
    fn test_sphere_bounding_box() {
        let sphere = Sphere {
            centre: Vector::new(0.0, 0.0, 0.0),
            radius: 1.0,
            material: Material::Dielectric {
                refractive_index: 1.5,
            },
        };

        let expected_box =
            AxisAlignedBoundingBox::new(Vector::new(-1.0, -1.0, -1.0), Vector::new(1.0, 1.0, 1.0));

        assert_eq!(sphere.bounding_box(0.0, 0.0), Some(expected_box));
    }

    #[test]
    fn test_moving_sphere_bounding_box() {
        let sphere = MovingSphere {
            centre_start: Vector::new(0.0, 0.0, 0.0),
            time_start: 0.0,
            centre_end: Vector::new(2.0, 2.0, 2.0),
            time_end: 2.0,
            radius: 1.0,
            material: Material::Dielectric {
                refractive_index: 1.5,
            },
        };

        let expected_box =
            AxisAlignedBoundingBox::new(Vector::new(-1.0, -1.0, -1.0), Vector::new(2.0, 2.0, 2.0));

        assert_eq!(sphere.bounding_box(0.0, 1.0), Some(expected_box));
    }

    #[test]
    fn test_sphere_texture_coords() {
        let centre = Vector::new(0.0, 0.0, 0.0);
        let radius = 1.0;

        let (row, col) = sphere_texture_coords(&Vector::new(0.0, 0.0, 1.0), &centre, radius);
        assert_approx_eq!(row, 0.5);
        assert_approx_eq!(col, 0.5);

        let (row, col) = sphere_texture_coords(&Vector::new(1.0, 0.0, 0.0), &centre, radius);
        assert_approx_eq!(row, 0.5);
        assert_approx_eq!(col, 0.75);

        let (row, col) = sphere_texture_coords(&Vector::new(-1.0, 0.0, 0.0), &centre, radius);
        assert_approx_eq!(row, 0.5);
        assert_approx_eq!(col, 0.25);

        let (row, col) = sphere_texture_coords(&Vector::new(0.0, 1.0, 0.0), &centre, radius);
        assert_approx_eq!(row, 1.0);
        assert_approx_eq!(col, 0.5);

        let (row, col) = sphere_texture_coords(&Vector::new(0.0, -1.0, 0.0), &centre, radius);
        assert_approx_eq!(row, 0.0);
        assert_approx_eq!(col, 0.5);
    }
}
