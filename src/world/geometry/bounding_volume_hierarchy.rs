use crate::camera::Ray;
use crate::data::assets::Assets;
use crate::sampling::uniform;
use crate::world::geometry::axis_aligned_bounding_box::AxisAlignedBoundingBox;
use crate::world::geometry::{Geometry, HitResult, Hittable};
use itertools::Itertools;
use std::cmp::Ordering;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BoundingVolumeHierarchyNode {
    left: Option<Box<Geometry>>,
    right: Option<Box<Geometry>>,
    bounding_box: AxisAlignedBoundingBox,
}

fn compare_setup(
    left: &Geometry,
    right: &Geometry,
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
    left: &Geometry,
    right: &Geometry,
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
    left: &Geometry,
    right: &Geometry,
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
    left: &Geometry,
    right: &Geometry,
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
    pub fn build(geometries: Vec<Geometry>, time_start: f64, time_end: f64) -> Geometry {
        let axis_choice = uniform::<u8>() % 3;
        let sorter = match axis_choice {
            0 => compare_box_by_x_axis,
            1 => compare_box_by_y_axis,
            _ => compare_box_by_z_axis,
        };

        let mut geometries: Vec<Geometry> = geometries
            .into_iter()
            .sorted_by(|left, right| sorter(left, right, time_start, time_end))
            .collect();

        let size = geometries.len();

        let (left, right) = match size {
            0 => (None, None),
            1 => (Some(Box::from(geometries.remove(0))), None),
            2 => (
                Some(Box::from(geometries.remove(0))),
                Some(Box::from(geometries.remove(0))),
            ),
            _ => {
                let mid = size / 2;

                let left_geometries: Vec<Geometry> = geometries.drain(0..mid).collect();
                let right_geometries = geometries;

                let left = Box::from(BoundingVolumeHierarchyNode::build(
                    left_geometries,
                    time_start,
                    time_end,
                ));
                let right = Box::from(BoundingVolumeHierarchyNode::build(
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

        Geometry::Bvh(Box::from(BoundingVolumeHierarchyNode {
            left,
            right,
            bounding_box,
        }))
    }
}

impl Hittable for BoundingVolumeHierarchyNode {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<HitResult> {
        if !self.bounding_box.intersection(&ray, tmin, tmax) {
            return None;
        }

        let hit_left = match &self.left {
            Some(geometry) => geometry.hit(&ray, tmin, tmax),
            None => None,
        };
        let hit_right = match &self.right {
            Some(geometry) => geometry.hit(&ray, tmin, tmax),
            None => None,
        };

        match (hit_left, hit_right) {
            (None, None) => None,
            (None, Some(hit)) => Some(hit),
            (Some(hit), None) => Some(hit),
            (Some(left_hit), Some(right_hit)) => {
                if left_hit.distance < right_hit.distance {
                    Some(left_hit)
                } else {
                    Some(right_hit)
                }
            }
        }
    }

    fn bounding_box(&self, _time_start: f64, _time_end: f64) -> Option<AxisAlignedBoundingBox> {
        Some(self.bounding_box.clone())
    }

    fn validate(&self, assets: &Assets) -> Result<(), anyhow::Error> {
        if let Some(geometry) = &self.left {
            geometry.validate(assets)?;
        }
        if let Some(geometry) = &self.right {
            geometry.validate(assets)?;
        }
        Ok(())
    }

    fn is_attractor(&self) -> bool {
        // We should not be using bounding boxes when importance sampling attractors
        false
    }
}
