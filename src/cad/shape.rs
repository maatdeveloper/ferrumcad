use crate::geometry::{Line, Circle, Geometry, Transform, BoundingBox};

#[derive(Debug, Clone)]
pub enum Shape {
    Line(Line),
    Circle(Circle),
}

impl Shape {
    pub fn as_line(&self) -> Option<&Line> {
        match self {
            Shape::Line(l) => Some(l),
            _ => None,
        }
    }
}

impl Geometry for Shape {
    fn bounding_box(&self) -> BoundingBox {
        match self {
            Shape::Line(l)   => l.bounding_box(),
            Shape::Circle(c) => c.bounding_box(),
        }
    }
}

impl Transform for Shape {
    fn translate(&self, dx: f64, dy: f64) -> Self {
        match self {
            Shape::Line(l)   => Shape::Line(l.translate(dx, dy)),
            Shape::Circle(c) => Shape::Circle(c.translate(dx, dy)),
        }
    }

    fn scale(&self, factor: f64) -> Self {
        match self {
            Shape::Line(l)   => Shape::Line(l.scale(factor)),
            Shape::Circle(c) => Shape::Circle(c.scale(factor)),
        }
    }
}
