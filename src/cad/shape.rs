use crate::geometry::{Line, Circle, BoundingBox, Geometry};

#[derive(Debug, Clone)]
pub enum Shape {
    Line(Line),
    Circle(Circle),
}

impl Shape {
    pub fn bounding_box(&self) -> BoundingBox {
        match self {
            Shape::Line(line)       => line.bounding_box(),
            Shape::Circle(circle)   => circle.bounding_box(),
        }
    }
}
