/* standard library */

/* external crates */

/* ferrumcad crates */
use crate::geometry::primitives::Point;

#[derive(Debug, Clone)]
pub struct BoundingBox {
    pub min: Point,
    pub max: Point,
}
