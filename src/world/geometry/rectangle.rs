use crate::camera::Ray;
use crate::data::assets::Assets;
use crate::data::vector::Vector;
use crate::sampling::uniform_between;
use crate::world::geometry::axis_aligned_bounding_box::AxisAlignedBoundingBox;
use crate::world::geometry::{Geometry, HitResult, Hittable};
use crate::world::materials::Material;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct XyRect {
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
    material: Material,
}

impl XyRect {
    pub fn build(
        x_lines: (f64, f64),
        y_lines: (f64, f64),
        z_plane: f64,
        material: Material,
    ) -> Geometry {
        let (x0, x1) = x_lines;
        let (y0, y1) = y_lines;
        Geometry::XyRect(Box::from(XyRect {
            x0,
            x1,
            y0,
            y1,
            k: z_plane,
            material,
        }))
    }
}

impl Hittable for XyRect {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<HitResult> {
        let distance = (self.k - ray.origin().z()) / ray.direction().z();

        if distance.is_nan() || distance < tmin || distance > tmax {
            return None;
        }

        let x = ray.origin().x() + distance * ray.direction().x();
        let y = ray.origin().y() + distance * ray.direction().y();

        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }

        Some(HitResult {
            distance,
            ray: *ray,
            point: ray.point(distance),
            surface_normal: Vector::new(0.0, 0.0, 1.0),
            material: self.material.clone(),
            texture_coords: (
                (x - self.x0) / (self.x1 - self.x0),
                (y - self.y0) / (self.y1 - self.y0),
            ),
        })
    }

    fn bounding_box(&self, _time_start: f64, _time_end: f64) -> Option<AxisAlignedBoundingBox> {
        Some(AxisAlignedBoundingBox::new(
            Vector::new(self.x0, self.y0, self.k - 0.0001),
            Vector::new(self.x1, self.y1, self.k + 0.0001),
        ))
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
            Some(hit) => {
                let area = (self.x1 - self.x0) * (self.y1 - self.y0);
                let distance_squared = hit.distance.powi(2) * direction.len_squared();
                let cosine = (Vector::dot(direction, &hit.face_normal()) / direction.len()).abs();

                distance_squared / (cosine * area)
            }
        }
    }

    fn random(&self, origin: &Vector) -> Vector {
        let random_point = Vector::new(
            uniform_between(self.x0, self.x1),
            uniform_between(self.y0, self.y1),
            self.k,
        );
        random_point - origin
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct XzRect {
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    k: f64,
    material: Material,
}

impl XzRect {
    pub fn build(
        x_lines: (f64, f64),
        z_lines: (f64, f64),
        y_plane: f64,
        material: Material,
    ) -> Geometry {
        let (x0, x1) = x_lines;
        let (z0, z1) = z_lines;
        Geometry::XzRect(Box::from(XzRect {
            x0,
            x1,
            z0,
            z1,
            k: y_plane,
            material,
        }))
    }
}

impl Hittable for XzRect {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<HitResult> {
        let distance = (self.k - ray.origin().y()) / ray.direction().y();

        if distance.is_nan() || distance < tmin || distance > tmax {
            return None;
        }

        let x = ray.origin().x() + distance * ray.direction().x();
        let z = ray.origin().z() + distance * ray.direction().z();

        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }

        Some(HitResult {
            distance,
            ray: *ray,
            point: ray.point(distance),
            surface_normal: Vector::new(0.0, 1.0, 0.0),
            material: self.material.clone(),
            texture_coords: (
                (x - self.x0) / (self.x1 - self.x0),
                (z - self.z0) / (self.z1 - self.z0),
            ),
        })
    }

    fn bounding_box(&self, _time_start: f64, _time_end: f64) -> Option<AxisAlignedBoundingBox> {
        Some(AxisAlignedBoundingBox::new(
            Vector::new(self.x0, self.k - 0.0001, self.z0),
            Vector::new(self.x1, self.k + 0.0001, self.z1),
        ))
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
            Some(hit) => {
                let area = (self.x1 - self.x0) * (self.z1 - self.z0);
                let distance_squared = hit.distance.powi(2) * direction.len_squared();
                let cosine = (Vector::dot(direction, &hit.face_normal()) / direction.len()).abs();

                distance_squared / (cosine * area)
            }
        }
    }

    fn random(&self, origin: &Vector) -> Vector {
        let random_point = Vector::new(
            uniform_between(self.x0, self.x1),
            self.k,
            uniform_between(self.z0, self.z1),
        );
        random_point - origin
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct YzRect {
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    k: f64,
    material: Material,
}

impl YzRect {
    pub fn build(
        y_lines: (f64, f64),
        z_lines: (f64, f64),
        x_plane: f64,
        material: Material,
    ) -> Geometry {
        let (y0, y1) = y_lines;
        let (z0, z1) = z_lines;
        Geometry::YzRect(Box::from(YzRect {
            y0,
            y1,
            z0,
            z1,
            k: x_plane,
            material,
        }))
    }
}

impl Hittable for YzRect {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<HitResult> {
        let distance = (self.k - ray.origin().x()) / ray.direction().x();

        if distance.is_nan() || distance < tmin || distance > tmax {
            return None;
        }

        let y = ray.origin().y() + distance * ray.direction().y();
        let z = ray.origin().z() + distance * ray.direction().z();

        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }

        Some(HitResult {
            distance,
            ray: *ray,
            point: ray.point(distance),
            surface_normal: Vector::new(1.0, 0.0, 0.0),
            material: self.material.clone(),
            texture_coords: (
                (y - self.y0) / (self.y1 - self.y0),
                (z - self.z0) / (self.z1 - self.z0),
            ),
        })
    }

    fn bounding_box(&self, _time_start: f64, _time_end: f64) -> Option<AxisAlignedBoundingBox> {
        Some(AxisAlignedBoundingBox::new(
            Vector::new(self.k - 0.0001, self.y0, self.z0),
            Vector::new(self.k + 0.0001, self.y1, self.z1),
        ))
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
            Some(hit) => {
                let area = (self.y1 - self.y0) * (self.z1 - self.z0);
                let distance_squared = hit.distance.powi(2) * direction.len_squared();
                let cosine = (Vector::dot(direction, &hit.face_normal()) / direction.len()).abs();

                distance_squared / (cosine * area)
            }
        }
    }

    fn random(&self, origin: &Vector) -> Vector {
        let random_point = Vector::new(
            self.k,
            uniform_between(self.y0, self.y1),
            uniform_between(self.z0, self.z1),
        );
        random_point - origin
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_xy_rect_hit() {
        let rect = XyRect {
            x0: 0.0,
            x1: 1.0,
            y0: 0.0,
            y1: 1.0,
            k: 0.0,
            material: Material::Dielectric {
                refractive_index: 1.5,
            },
        };
        let ray = Ray::new(Vector::new(0.5, 0.5, 1.0), Vector::new(0.0, 0.0, -1.0), 0.0);

        let hit_result = rect.hit(&ray, 0.0, core::f64::MAX).unwrap();
        assert_approx_eq!(hit_result.distance, 1.0);
    }

    #[test]
    fn test_xy_rect_surface_normal() {
        let rect = XyRect {
            x0: 0.0,
            x1: 1.0,
            y0: 0.0,
            y1: 1.0,
            k: 0.0,
            material: Material::Dielectric {
                refractive_index: 1.5,
            },
        };
        let ray = Ray::new(Vector::new(0.5, 0.5, 1.0), Vector::new(0.0, 0.0, -1.0), 0.0);

        let hit_result = rect.hit(&ray, 0.0, core::f64::MAX).unwrap();
        assert_eq!(hit_result.surface_normal, Vector::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_xy_rect_bounding_box() {
        let rect = XyRect {
            x0: 0.0,
            x1: 1.0,
            y0: 0.0,
            y1: 1.0,
            k: 0.0,
            material: Material::Dielectric {
                refractive_index: 1.5,
            },
        };

        let expected_box = AxisAlignedBoundingBox::new(
            Vector::new(0.0, 0.0, -0.0001),
            Vector::new(1.0, 1.0, 0.0001),
        );

        assert_eq!(rect.bounding_box(0.0, 0.0), Some(expected_box));
    }

    #[test]
    fn test_xy_rect_texture_coords() {
        let rect = XyRect {
            x0: 0.0,
            x1: 1.0,
            y0: 0.0,
            y1: 1.0,
            k: 0.0,
            material: Material::Dielectric {
                refractive_index: 1.5,
            },
        };
        let ray = Ray::new(Vector::new(0.5, 0.5, 1.0), Vector::new(0.0, 0.0, -1.0), 0.0);

        let hit_result = rect.hit(&ray, 0.0, core::f64::MAX).unwrap();
        let (u, v) = hit_result.texture_coords;
        assert_approx_eq!(u, 0.5);
        assert_approx_eq!(v, 0.5);
    }

    #[test]
    fn test_xz_rect_hit() {
        let rect = XzRect {
            x0: 0.0,
            x1: 1.0,
            z0: 0.0,
            z1: 1.0,
            k: 0.0,
            material: Material::Dielectric {
                refractive_index: 1.5,
            },
        };
        let ray = Ray::new(Vector::new(0.5, 1.0, 0.5), Vector::new(0.0, -1.0, 0.0), 0.0);

        let hit_result = rect.hit(&ray, 0.0, core::f64::MAX).unwrap();
        assert_approx_eq!(hit_result.distance, 1.0);
    }

    #[test]
    fn test_xz_rect_surface_normal() {
        let rect = XzRect {
            x0: 0.0,
            x1: 1.0,
            z0: 0.0,
            z1: 1.0,
            k: 0.0,
            material: Material::Dielectric {
                refractive_index: 1.5,
            },
        };
        let ray = Ray::new(Vector::new(0.5, 1.0, 0.5), Vector::new(0.0, -1.0, 0.0), 0.0);

        let hit_result = rect.hit(&ray, 0.0, core::f64::MAX).unwrap();
        assert_eq!(hit_result.surface_normal, Vector::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn test_xz_rect_bounding_box() {
        let rect = XzRect {
            x0: 0.0,
            x1: 1.0,
            z0: 0.0,
            z1: 1.0,
            k: 0.0,
            material: Material::Dielectric {
                refractive_index: 1.5,
            },
        };

        let expected_box = AxisAlignedBoundingBox::new(
            Vector::new(0.0, -0.0001, 0.0),
            Vector::new(1.0, 0.0001, 1.0),
        );

        assert_eq!(rect.bounding_box(0.0, 0.0), Some(expected_box));
    }

    #[test]
    fn test_xz_rect_texture_coords() {
        let rect = XzRect {
            x0: 0.0,
            x1: 1.0,
            z0: 0.0,
            z1: 1.0,
            k: 0.0,
            material: Material::Dielectric {
                refractive_index: 1.5,
            },
        };
        let ray = Ray::new(Vector::new(0.5, 1.0, 0.5), Vector::new(0.0, -1.0, 0.0), 0.0);

        let hit_result = rect.hit(&ray, 0.0, core::f64::MAX).unwrap();
        let (u, v) = hit_result.texture_coords;
        assert_approx_eq!(u, 0.5);
        assert_approx_eq!(v, 0.5);
    }

    #[test]
    fn test_yz_rect_hit() {
        let rect = YzRect {
            y0: 0.0,
            y1: 1.0,
            z0: 0.0,
            z1: 1.0,
            k: 0.0,
            material: Material::Dielectric {
                refractive_index: 1.5,
            },
        };
        let ray = Ray::new(Vector::new(1.0, 0.5, 0.5), Vector::new(-1.0, 0.0, 0.0), 0.0);

        let hit_result = rect.hit(&ray, 0.0, core::f64::MAX).unwrap();
        assert_approx_eq!(hit_result.distance, 1.0);
    }

    #[test]
    fn test_yz_rect_surface_normal() {
        let rect = YzRect {
            y0: 0.0,
            y1: 1.0,
            z0: 0.0,
            z1: 1.0,
            k: 0.0,
            material: Material::Dielectric {
                refractive_index: 1.5,
            },
        };
        let ray = Ray::new(Vector::new(1.0, 0.5, 0.5), Vector::new(-1.0, 0.0, 0.0), 0.0);

        let hit_result = rect.hit(&ray, 0.0, core::f64::MAX).unwrap();
        assert_eq!(hit_result.surface_normal, Vector::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn test_yz_rect_bounding_box() {
        let rect = YzRect {
            y0: 0.0,
            y1: 1.0,
            z0: 0.0,
            z1: 1.0,
            k: 0.0,
            material: Material::Dielectric {
                refractive_index: 1.5,
            },
        };

        let expected_box = AxisAlignedBoundingBox::new(
            Vector::new(-0.0001, 0.0, 0.0),
            Vector::new(0.0001, 1.0, 1.0),
        );

        assert_eq!(rect.bounding_box(0.0, 0.0), Some(expected_box));
    }

    #[test]
    fn test_yz_rect_texture_coords() {
        let rect = YzRect {
            y0: 0.0,
            y1: 1.0,
            z0: 0.0,
            z1: 1.0,
            k: 0.0,
            material: Material::Dielectric {
                refractive_index: 1.5,
            },
        };
        let ray = Ray::new(Vector::new(1.0, 0.5, 0.5), Vector::new(-1.0, 0.0, 0.0), 0.0);

        let hit_result = rect.hit(&ray, 0.0, core::f64::MAX).unwrap();
        let (u, v) = hit_result.texture_coords;
        assert_approx_eq!(u, 0.5);
        assert_approx_eq!(v, 0.5);
    }
}
