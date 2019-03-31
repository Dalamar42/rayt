use camera::Ray;
use itertools::Itertools;
use rand::prelude::*;
use std::cmp::Ordering;
use world::geometry::axis_aligned_bounding_box::AxisAlignedBoundingBox;
use world::geometry::{Geometry, HitResult};

#[derive(Serialize, Deserialize)]
pub struct BoundingVolumeHierarchyNode {
    left: Option<Box<dyn Geometry>>,
    right: Option<Box<dyn Geometry>>,
    bounding_box: AxisAlignedBoundingBox,
}

fn compare_setup(
    left: &dyn Geometry,
    right: &dyn Geometry,
    time_start: f64,
    time_end: f64,
) -> (AxisAlignedBoundingBox, AxisAlignedBoundingBox) {
    let left_box = left.bounding_box(time_start, time_end);
    let right_box = right.bounding_box(time_start, time_end);

    let left_box = match left_box {
        Some(bounding_box) => bounding_box,
        None => panic!("Geometries with no bounding boxes are not supported"),
    };
    let right_box = match right_box {
        Some(bounding_box) => bounding_box,
        None => panic!("Geometries with no bounding boxes are not supported"),
    };

    (left_box, right_box)
}

fn compare_box_by_x_axis(
    left: &dyn Geometry,
    right: &dyn Geometry,
    time_start: f64,
    time_end: f64,
) -> Ordering {
    let (left_box, right_box) = compare_setup(left, right, time_start, time_end);

    // Should never get a NaN here. Panic if we do
    left_box
        .min()
        .x()
        .partial_cmp(&right_box.min().x())
        .unwrap()
}

fn compare_box_by_y_axis(
    left: &dyn Geometry,
    right: &dyn Geometry,
    time_start: f64,
    time_end: f64,
) -> Ordering {
    let (left_box, right_box) = compare_setup(left, right, time_start, time_end);

    // Should never get a NaN here. Panic if we do
    left_box
        .min()
        .y()
        .partial_cmp(&right_box.min().y())
        .unwrap()
}

fn compare_box_by_z_axis(
    left: &dyn Geometry,
    right: &dyn Geometry,
    time_start: f64,
    time_end: f64,
) -> Ordering {
    let (left_box, right_box) = compare_setup(left, right, time_start, time_end);

    // Should never get a NaN here. Panic if we do
    left_box
        .min()
        .z()
        .partial_cmp(&right_box.min().z())
        .unwrap()
}

impl BoundingVolumeHierarchyNode {
    pub fn new(
        geometries: Vec<Box<dyn Geometry>>,
        time_start: f64,
        time_end: f64,
    ) -> BoundingVolumeHierarchyNode {
        let mut rng = rand::thread_rng();
        let axis_choice = rng.gen::<u8>() % 3;
        let sorter = match axis_choice {
            0 => compare_box_by_x_axis,
            1 => compare_box_by_y_axis,
            _ => compare_box_by_z_axis,
        };

        let mut geometries: Vec<Box<dyn Geometry>> = geometries
            .into_iter()
            .sorted_by(|left, right| sorter(left.as_ref(), right.as_ref(), time_start, time_end))
            .collect();

        let size = geometries.len();

        let (left, right) = match size {
            0 => (None, None),
            1 => (Some(geometries.remove(0)), None),
            2 => (Some(geometries.remove(0)), Some(geometries.remove(0))),
            _ => {
                let mid = size / 2;

                let left_geometries: Vec<Box<dyn Geometry>> = geometries.drain(0..mid).collect();
                let right_geometries = geometries;

                let left: Box<dyn Geometry> = Box::from(BoundingVolumeHierarchyNode::new(
                    left_geometries,
                    time_start,
                    time_end,
                ));
                let right: Box<dyn Geometry> = Box::from(BoundingVolumeHierarchyNode::new(
                    right_geometries,
                    time_start,
                    time_end,
                ));

                (Some(left), Some(right))
            }
        };

        let bounding_box = match (&left, &right) {
            (Some(left_geometry), Some(right_geometry)) => AxisAlignedBoundingBox::surrounding(
                &left_geometry.bounding_box(time_start, time_end),
                &right_geometry.bounding_box(time_start, time_end),
            ),
            (Some(left_geometry), None) => left_geometry.bounding_box(time_start, time_end),
            (None, Some(right_geometry)) => right_geometry.bounding_box(time_start, time_end),
            (None, None) => None,
        };
        let bounding_box = match bounding_box {
            Some(bounding_box) => bounding_box,
            None => panic!("Geometries with no bounding boxes are not supported"),
        };

        BoundingVolumeHierarchyNode {
            left,
            right,
            bounding_box,
        }
    }
}

#[typetag::serde]
impl Geometry for BoundingVolumeHierarchyNode {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> HitResult {
        if !self.bounding_box.intersection(&ray, tmin, tmax) {
            return HitResult::Miss;
        }

        let hit_left = match &self.left {
            Some(geometry) => geometry.hit(&ray, tmin, tmax),
            None => HitResult::Miss,
        };
        let hit_right = match &self.right {
            Some(geometry) => geometry.hit(&ray, tmin, tmax),
            None => HitResult::Miss,
        };

        match (hit_left, hit_right) {
            (HitResult::Miss, HitResult::Miss) => HitResult::Miss,
            (
                HitResult::Miss,
                HitResult::Hit {
                    distance,
                    ray,
                    point,
                    surface_normal,
                    material,
                    texture_coords,
                },
            ) => HitResult::Hit {
                distance,
                ray,
                point,
                surface_normal,
                material,
                texture_coords,
            },
            (
                HitResult::Hit {
                    distance,
                    ray,
                    point,
                    surface_normal,
                    material,
                    texture_coords,
                },
                HitResult::Miss,
            ) => HitResult::Hit {
                distance,
                ray,
                point,
                surface_normal,
                material,
                texture_coords,
            },
            (
                HitResult::Hit {
                    distance: dl,
                    ray: rl,
                    point: pl,
                    surface_normal: sl,
                    material: ml,
                    texture_coords: tl,
                },
                HitResult::Hit {
                    distance: dr,
                    ray: rr,
                    point: pr,
                    surface_normal: sr,
                    material: mr,
                    texture_coords: tr,
                },
            ) => {
                if dl < dr {
                    HitResult::Hit {
                        distance: dl,
                        ray: rl,
                        point: pl,
                        surface_normal: sl,
                        material: ml,
                        texture_coords: tl,
                    }
                } else {
                    HitResult::Hit {
                        distance: dr,
                        ray: rr,
                        point: pr,
                        surface_normal: sr,
                        material: mr,
                        texture_coords: tr,
                    }
                }
            }
        }
    }

    fn bounding_box(&self, _time_start: f64, _time_end: f64) -> Option<AxisAlignedBoundingBox> {
        Some(self.bounding_box.clone())
    }
}
