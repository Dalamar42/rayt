use crate::camera::Ray;
use crate::data::assets::Assets;
use crate::data::vector::Vector;
use crate::sampling::random_point_in_unit_sphere;
use crate::world::materials::ScatterResult;
use crate::world::texture::Texture;

pub fn scatter(
    albedo: &Texture,
    ray: &Ray,
    hit_point: &Vector,
    texture_coords: (f64, f64),
    assets: &Assets,
) -> Option<ScatterResult> {
    let scattered = Ray::new(*hit_point, random_point_in_unit_sphere(), ray.time());
    let attenuation = albedo.value(texture_coords, hit_point, assets);
    Some(ScatterResult::new(scattered, attenuation, 1.0))
}
