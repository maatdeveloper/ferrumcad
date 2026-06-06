/* standard library */

/* external crates */
use serde::{Serialize, Deserialize};

/* ferrumcad crates */
use super::Point;
use crate::geometry::bounding::BoundingBox;
use crate::geometry::traits::{Geometry, Transform};
use crate::geometry::utils::{EPS, approx_eq, approx_zero};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Line {
    pub start: Point,
    pub end: Point,
}

#[derive(Debug, Clone)]
pub enum Intersection {
    None,
    Point(Point),
    Overlap(Line),
}

impl Line {
    pub fn length(&self) -> f64 {
        self.start.distance(&self.end)
    }

    pub fn midpoint(&self) -> Point {
        Point {
            x: (self.start.x + self.end.x) / 2.0,
            y: (self.start.y + self.end.y) / 2.0,
        }
    }

    pub fn intersects(&self, other: &Line) -> bool {
        let p1 = self.start;
        let q1 = self.end;
        let p2 = other.start;
        let q2 = other.end;

        let o1 = orientation(p1, q1, p2);
        let o2 = orientation(p1, q1, q2);
        let o3 = orientation(p2, q2, p1);
        let o4 = orientation(p2, q2, q1);

        if o1 != o2 && o3 != o4 {
            return true;
        }

        if o1 == 0 && on_segment(p1, p2, q1) {
            return true;
        }
        if o2 == 0 && on_segment(p1, q2, q1) {
            return true;
        }
        if o3 == 0 && on_segment(p2, p1, q2) {
            return true;
        }
        if o4 == 0 && on_segment(p2, q1, q2) {
            return true;
        }

        false
    }

    pub fn intersection(&self, other: &Line) -> Intersection {
        let x1 = self.start.x;
        let y1 = self.start.y;
        let x2 = self.end.x;
        let y2 = self.end.y;

        let x3 = other.start.x;
        let y3 = other.start.y;
        let x4 = other.end.x;
        let y4 = other.end.y;

        let denom = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);

        if approx_zero(denom) {
            if colinear(self.start, self.end, other.start) {
                return overlap(self, other);
            }

            return Intersection::None;
        }

        let px = ((x1 * y2 - y1 * x2) * (x3 - x4) - (x1 - x2) * (x3 * y4 - y3 * x4)) / denom;
        let py = ((x1 * y2 - y1 * x2) * (y3 - y4) - (y1 - y2) * (x3 * y4 - y3 * x4)) / denom;

        let p = Point { x: px, y: py };

        if on_segment(self.start, p, self.end) && on_segment(other.start, p, other.end) {
            return Intersection::Point(p);
        }

        Intersection::None
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

impl Transform for Line {
    fn translate(&self, dx: f64, dy: f64) -> Self {
        Line {
            start: self.start.translate(dx, dy),
            end: self.end.translate(dx, dy),
        }
    }

    fn scale(&self, factor: f64) -> Self {
        Line {
            start: self.start.scale(factor),
            end: self.end.scale(factor),
        }
    }
}

fn orientation(p: Point, q: Point, r: Point) -> i32 {
    let val = (q.y - p.y) * (r.x - q.x) - (q.x - p.x) * (r.y - q.y);

    if val.abs() < EPS {
        0 // colinear
    } else if val > 0.0 {
        1 // horario
    } else {
        2 // anti-horario
    }
}

fn on_segment(p: Point, q: Point, r: Point) -> bool {
    q.x <= p.x.max(r.x) + EPS
        && q.x >= p.x.min(r.x) - EPS
        && q.y <= p.y.max(r.y) + EPS
        && q.y >= p.y.min(r.y) - EPS
}

fn colinear(p: Point, q: Point, r: Point) -> bool {
    let val = (q.y - p.y) * (r.x - q.x) - (q.x - p.x) * (r.y - q.y);

    approx_zero(val)
}

fn overlap(l1: &Line, l2: &Line) -> Intersection {
    let l1_min = l1.start.x.min(l1.end.x);
    let l1_max = l1.start.x.max(l1.end.x);

    let l2_min = l2.start.x.min(l2.end.x);
    let l2_max = l2.start.x.max(l2.end.x);

    let overlap_start = l1_min.max(l2_min);
    let overlap_end = l1_max.min(l2_max);

    if overlap_start > overlap_end + EPS {
        return Intersection::None;
    }

    if approx_eq(overlap_start, overlap_end) {
        return Intersection::Point(Point {
            x: overlap_start,
            y: l1.start.y,
        });
    }

    Intersection::Overlap(Line {
        start: Point {
            x: overlap_start,
            y: l1.start.y,
        },
        end: Point {
            x: overlap_end,
            y: l1.start.y,
        },
    })
}
