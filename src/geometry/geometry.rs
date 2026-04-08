use super::BoundingBox;

pub trait Geometry {
    fn bounding_box(&self) -> BoundingBox;
}
