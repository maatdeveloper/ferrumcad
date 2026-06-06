/* standard library */

/* external crates */
use serde::{Serialize, Deserialize};

/* ferrumcad crates */
use super::Point;
use crate::geometry::bounding::BoundingBox;
use crate::geometry::traits::{Geometry, Transform};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
    pub diameter: f64,
}

impl Circle {
    pub fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powi(2)
    }
}

impl Geometry for Circle {
    fn bounding_box(&self) -> BoundingBox {
        BoundingBox {
            min: Point {
                x: self.center.x - self.radius,
                y: self.center.y - self.radius,
            },
            max: Point {
                x: self.center.x + self.radius,
                y: self.center.y + self.radius,
            },
        }
    }
}

impl Transform for Circle {
    fn translate(&self, dx: f64, dy: f64) -> Self {
        Circle {
            center: self.center.translate(dx, dy),
            radius: self.radius,
            diameter: self.diameter,
        }
    }

    fn scale(&self, factor: f64) -> Self {
        Circle {
            center: self.center.scale(factor),
            radius: self.radius * factor,
            diameter: self.diameter * factor,
        }
    }
}
