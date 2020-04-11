use crate::config::ConfigSave;

mod basic;
mod cornell_box;
mod cornell_metal;
mod cornell_smoke;
mod cornell_sphere;
mod cover;
mod next_week_final;
mod perlin;
mod planets;
mod simple_light;

arg_enum! {
    #[derive(Debug)]
    pub enum Scene {
        Basic,
        Cover,
        CoverWithMotionBlur,
        CoverWithChecker,
        Perlin,
        Planets,
        SimpleLight,
        CornellBox,
        CornellSmoke,
        CornellMetal,
        CornellSphere,
        NextWeekFinal,
    }
}

pub fn build_scene_config(scene: &Scene) -> Result<ConfigSave, anyhow::Error> {
    match scene {
        Scene::Basic => basic::build(),
        Scene::Cover => cover::build(false, false),
        Scene::CoverWithMotionBlur => cover::build(true, false),
        Scene::CoverWithChecker => cover::build(true, true),
        Scene::Perlin => perlin::build(),
        Scene::Planets => planets::build(),
        Scene::SimpleLight => simple_light::build(),
        Scene::CornellBox => cornell_box::build(),
        Scene::CornellSmoke => cornell_smoke::build(),
        Scene::CornellMetal => cornell_metal::build(),
        Scene::CornellSphere => cornell_sphere::build(),
        Scene::NextWeekFinal => next_week_final::build(),
    }
}
