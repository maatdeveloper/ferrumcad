use super::Point;
use super::BoundingBox;

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

    pub fn bounding_box(&self) -> BoundingBox {
        // largura e altura sao 2r
        let circle_box = self.radius * 2.0;

        BoundingBox {
            min: Point {
                x: 0.0,
                y: 0.0,
            },
            max: Point {
                x: circle_box,
                y: circle_box,
            },
        }
    }
}
