use super::Point;

#[derive(Debug, Clone)]
pub struct BoundingBox {
    pub min: Point,
    pub max: Point,
}
