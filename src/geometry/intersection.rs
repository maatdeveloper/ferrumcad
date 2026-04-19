use super::{Point, Line};

#[derive(Debug)]
pub enum Intersection {
    None,
    Point(Point),
    Overlap(Line),
}
