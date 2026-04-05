#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64
}

impl Point {
    pub fn distance(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
    pub fn translate(&self, dx: f64, dy: f64) -> Point {
        Point {
            x: (self.x + dx),
            y: (self.y + dy),
        }
    }
    pub fn scale(&self, factor: f64) -> Point {
        Point {
            x: (self.x) * factor,
            y: (self.y) * factor,
        }
    }
}
