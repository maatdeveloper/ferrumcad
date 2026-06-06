/* standard library */

/* external crates */
use serde::{Serialize, Deserialize};

/* ferrumcad crates */
use super::Shape;
use crate::geometry::primitives::{Line, Point};
use crate::persistence::{save_model, load_model};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Model {
    pub shapes: Vec<Shape>,
}

impl Model {
    pub fn new() -> Self {
        Self { shapes: Vec::new() }
    }

    pub fn add_line(&mut self, start: Point, end: Point) {
        self.shapes.push(Shape::Line(Line { start, end }));
    }

    pub fn save(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        save_model(path, self)
    }

    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        load_model(path)
    }
}

impl Default for Model {
    fn default() -> Self {
        Self {
            shapes: Vec::new(),
        }
    }
}
