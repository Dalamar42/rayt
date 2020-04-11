use crate::camera::Ray;
use crate::data::assets::Assets;
use crate::data::vector::Vector;
use crate::onb::Onb;
use crate::sampling::uniform;
use crate::world::geometry::axis_aligned_bounding_box::AxisAlignedBoundingBox;
use crate::world::geometry::{Geometry, HitResult, Hittable};
use crate::world::materials::Material;
use std::f64::consts::PI;

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
    if tmin <= t && t <= tmax {
        return Some(t);
    }

    let t = (-b + discriminant.sqrt()) / (2.0 * a);
    if tmin <= t && t <= tmax {
        return Some(t);
    }

    None
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

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Sphere {
    centre: Vector,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub fn build(centre: Vector, radius: f64, material: Material) -> Geometry {
        Geometry::Sphere(Box::from(Sphere {
            centre,
            radius,
            material,
        }))
    }

    fn surface_normal(&self, ray: &Ray, distance: f64) -> Vector {
        // We divide by radius instead of taking the unit vector so that a negative
        // radius sphere will have a surface normal that points inward
        (ray.point(distance) - self.centre) / self.radius
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<HitResult> {
        sphere_hit(ray, &self.centre, self.radius, tmin, tmax).map(|distance| {
            let point = ray.point(distance);
            let surface_normal = self.surface_normal(&ray, distance);

            let texture_coords = sphere_texture_coords(&point, &self.centre, self.radius);

            HitResult {
                distance,
                ray: *ray,
                point,
                surface_normal,
                material: self.material.clone(),
                texture_coords,
            }
        })
    }

    fn bounding_box(&self, _time_start: f64, _time_end: f64) -> Option<AxisAlignedBoundingBox> {
        sphere_bounding_box(&self.centre, self.radius)
    }

    fn validate(&self, assets: &Assets) -> Result<(), anyhow::Error> {
        Ok(self.material.validate(assets)?)
    }

    fn is_attractor(&self) -> bool {
        self.material.is_attractor()
    }

    fn pdf_value(&self, origin: &Vector, direction: &Vector) -> f64 {
        let hit = self.hit(&Ray::new(*origin, *direction, 0.0), 0.001, std::f64::MAX);
        match hit {
            None => 0.0,
            Some(_hit) => {
                let cp = self.centre - origin;
                let distance_ratio = self.radius.powi(2) / cp.len_squared();
                if distance_ratio >= 1.0 {
                    // This means origin is inside the sphere. Any ray will hit the sphere
                    return 1.0;
                }

                let cos_theta_max = f64::sqrt(1.0 - distance_ratio);
                let solid_angle = 2.0 * PI * (1.0 - cos_theta_max);

                1.0 / solid_angle
            }
        }
    }

    fn random(&self, origin: &Vector) -> Vector {
        let cp = self.centre - origin;
        let distance_ratio = self.radius.powi(2) / cp.len_squared();
        if distance_ratio >= 1.0 {
            // This means origin is inside the sphere. Any ray will hit the sphere
            return Vector::new(uniform(), uniform(), uniform());
        }

        let onb = Onb::build_from_w(&cp);
        onb.local_from_vec(&random_to_sphere(distance_ratio))
    }
}

pub fn random_to_sphere(distance_ratio: f64) -> Vector {
    let r1 = uniform::<f64>();
    let r2 = uniform::<f64>();

    let cos_theta_max = f64::sqrt(1.0 - distance_ratio);
    let z = 1.0 + r2 * (cos_theta_max - 1.0);

    let phi = 2.0 * PI * r1;
    let x = f64::cos(phi) * f64::sqrt(1.0 - z.powi(2));
    let y = f64::sin(phi) * f64::sqrt(1.0 - z.powi(2));

    Vector::new(x, y, z)
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct MovingSphere {
    centre_start: Vector,
    time_start: f64,
    centre_end: Vector,
    time_end: f64,
    radius: f64,
    material: Material,
}

impl MovingSphere {
    pub fn build(
        centre_start: Vector,
        time_start: f64,
        centre_end: Vector,
        time_end: f64,
        radius: f64,
        material: Material,
    ) -> Geometry {
        Geometry::MovingSphere(Box::from(MovingSphere {
            centre_start,
            time_start,
            centre_end,
            time_end,
            radius,
            material,
        }))
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

impl Hittable for MovingSphere {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<HitResult> {
        let centre = self.centre(ray.time());
        sphere_hit(ray, &centre, self.radius, tmin, tmax).map(|distance| {
            let point = ray.point(distance);
            let surface_normal = self.surface_normal(&ray, distance);

            let texture_coords = sphere_texture_coords(&point, &centre, self.radius);

            HitResult {
                distance,
                ray: *ray,
                point,
                surface_normal,
                material: self.material.clone(),
                texture_coords,
            }
        })
    }

    fn bounding_box(&self, time_start: f64, time_end: f64) -> Option<AxisAlignedBoundingBox> {
        let box_start = sphere_bounding_box(&self.centre(time_start), self.radius);
        let box_end = sphere_bounding_box(&self.centre(time_end), self.radius);

        AxisAlignedBoundingBox::surrounding(&box_start, &box_end)
    }

    fn validate(&self, assets: &Assets) -> Result<(), anyhow::Error> {
        self.material.validate(assets)
    }

    fn is_attractor(&self) -> bool {
        false
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

        let hit_result = sphere.hit(&ray, 0.0, core::f64::MAX).unwrap();
        assert_approx_eq!(hit_result.distance, 1.0);
    }

    #[test]
    fn test_sphere_interior_hit() {
        let sphere = Sphere {
            centre: Vector::new(0.0, 0.0, 0.0),
            radius: 1.0,
            material: Material::Dielectric {
                refractive_index: 1.5,
            },
        };
        let ray = Ray::new(Vector::new(-2.0, 0.0, 0.0), Vector::new(1.0, 0.0, 0.0), 0.0);

        let hit_result = sphere.hit(&ray, 1.0001, core::f64::MAX).unwrap();
        assert_approx_eq!(hit_result.distance, 3.0);
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

        let hit_result = sphere.hit(&ray, 0.0, core::f64::MAX).unwrap();
        assert_approx_eq!(hit_result.distance, 1.0);
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

        let hit_result = sphere.hit(&ray, 0.0, core::f64::MAX).unwrap();
        assert_eq!(hit_result.surface_normal, Vector::new(-1.0, 0.0, 0.0));
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

        let hit_result = sphere.hit(&ray, 0.0, core::f64::MAX).unwrap();
        assert_eq!(hit_result.surface_normal, Vector::new(1.0, 0.0, 0.0));
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
