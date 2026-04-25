use crate::geometry::bounding::{BoundingBox};

pub trait Geometry {
    fn bounding_box(&self) -> BoundingBox;
}
