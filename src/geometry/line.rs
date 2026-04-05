use super::Point;
use super::BoundingBox;

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

    pub fn bounding_box(&self) -> BoundingBox {
        // canto inferior esquerdo
        let inf_esq = (self.start.x) * (self.start.y);

        // canto superior direito
        let sup_dir = (self.end.x) * (self.end.y);

        BoundingBox {
            min: Point {
                x: inf_esq,
                y: inf_esq,
            },
            max: Point {
                x: sup_dir,
                y: sup_dir,
            },
        }
    }
}
