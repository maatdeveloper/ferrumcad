use super::{Point, BoundingBox, Geometry};


#[derive(Debug, Clone)]
pub struct Line {
    pub start: Point,
    pub end: Point,
}

impl Line {
    pub fn lenght(&self) -> f64 {
        self.start.distance(&self.end)
    }

    pub fn midpoint(&self) -> Point {
        Point {
            x: (self.start.x + self.end.x) / 2.0,
            y: (self.start.y + self.end.y) / 2.0,
        }
    }

    pub fn translate(&self, dx: f64, dy: f64) -> Line {
        Line {
            start: Point {
                x: (self.start.x + dx),
                y: (self.start.y + dy),
            },
            end: Point {
                x: (self.end.x + dx),
                y: (self.end.y + dy),
            },
        }
    }
}

impl Geometry for Line {
    fn bounding_box(&self) -> BoundingBox {
        BoundingBox {
            min: Point {
                x: self.start.x.min(self.end.x),
                y: self.start.y.min(self.end.y),
            },
            max: Point {
                x: self.start.x.max(self.end.x),
                y: self.start.y.max(self.end.y),
            },
        }
    }
}
