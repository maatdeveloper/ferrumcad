use crate::geometry::primitives::{Point, Line};
use super::Shape;

#[derive(Debug)]
pub struct Model {
    pub shapes: Vec<Shape>,
}

impl Model {
    pub fn new() -> Self {
        Self {
            shapes: Vec::new(),
        }
    }

    pub fn add_line(&mut self, start: Point, end: Point) {
        self.shapes.push(Shape::Line(Line { start, end }));
    }
}
