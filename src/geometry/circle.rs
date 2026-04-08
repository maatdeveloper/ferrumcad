use super::{Point, BoundingBox, Geometry};

#[derive(Debug, Clone)]
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
