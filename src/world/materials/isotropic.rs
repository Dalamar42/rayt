use crate::camera::Ray;
use crate::data::assets::Assets;
use crate::sampling::random_point_in_unit_sphere;
use crate::world::geometry::HitResult;
use crate::world::materials::ScatterResult;
use crate::world::texture::Texture;

pub fn scatter(albedo: &Texture, hit: &HitResult, assets: &Assets) -> Option<ScatterResult> {
    let scattered = Ray::new(hit.point, random_point_in_unit_sphere(), hit.ray.time());
    let attenuation = albedo.value(hit.texture_coords, &hit.point, assets);
    Some(ScatterResult::specular(attenuation, scattered))
}
