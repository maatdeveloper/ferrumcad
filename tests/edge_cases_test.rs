/* standard library */

/* external crates */

/* ferrumcad crates */
use ferrumcad::geometry::utils::approx_zero;
use ferrumcad::geometry::{Geometry, Intersection, Line, Point};

#[test]
fn test_intersection_at_endpoint() {
    let l1 = Line {
        start: Point { x: 0.0, y: 0.0 },
        end: Point { x: 5.0, y: 5.0 },
    };

    let l2 = Line {
        start: Point { x: 5.0, y: 5.0 },
        end: Point { x: 10.0, y: 0.0 },
    };

    match l1.intersection(&l2) {
        Intersection::Point(p) => {
            assert!(approx_zero(p.x - 5.0));
            assert!(approx_zero(p.y - 5.0));
        }
        _ => panic!("Expected endpoint intersection"),
    }
}

#[test]
fn test_colinear_no_overlap() {
    let l1 = Line {
        start: Point { x: 0.0, y: 0.0 },
        end: Point { x: 5.0, y: 0.0 },
    };

    let l2 = Line {
        start: Point { x: 6.0, y: 0.0 },
        end: Point { x: 10.0, y: 0.0 },
    };

    match l1.intersection(&l2) {
        Intersection::None => {}
        _ => panic!("Expected no instersection"),
    }
}

#[test]
fn test_full_overlap() {
    let l1 = Line {
        start: Point { x: 0.0, y: 0.0 },
        end: Point { x: 10.0, y: 0.0 },
    };

    let l2 = Line {
        start: Point { x: 2.0, y: 0.0 },
        end: Point { x: 8.0, y: 0.0 },
    };

    match l1.intersection(&l2) {
        Intersection::Overlap(line) => {
            assert!(approx_zero(line.start.x - 2.0));
            assert!(approx_zero(line.end.x - 8.0));
        }
        _ => panic!("Expected overlap"),
    }
}

#[test]
fn test_almost_parallel() {
    let l1 = Line {
        start: Point { x: 0.0, y: 0.0 },
        end: Point {
            x: 10.0,
            y: 0.000000001,
        },
    };

    let l2 = Line {
        start: Point { x: 0.0, y: 1.0 },
        end: Point {
            x: 10.0,
            y: 1.000000001,
        },
    };

    match l1.intersection(&l2) {
        Intersection::None => {}
        _ => panic!("Should be treated as parallel"),
    }
}

#[test]
fn test_degenerate_line() {
    let l1 = Line {
        start: Point { x: 1.0, y: 1.0 },
        end: Point { x: 1.0, y: 1.0 },
    };

    let l2 = Line {
        start: Point { x: 0.0, y: 0.0 },
        end: Point { x: 2.0, y: 2.0 },
    };

    match l1.intersection(&l2) {
        Intersection::Point(p) => {
            assert!(approx_zero(p.x - 1.0));
            assert!(approx_zero(p.y - 1.0));
        }
        _ => panic!("Expected point intersection"),
    }
}

#[test]
fn test_lines_intersect_but_segments_do_not() {
    let l1 = Line {
        start: Point { x: 0.0, y: 0.0 },
        end: Point { x: 1.0, y: 1.0 },
    };

    let l2 = Line {
        start: Point { x: 2.0, y: 0.0 },
        end: Point { x: 2.0, y: 5.0 },
    };

    match l1.intersection(&l2) {
        Intersection::None => {}
        _ => panic!("Expected no segment intersection"),
    }
}

#[test]
fn test_bounding_box_point_line() {
    let line = Line {
        start: Point { x: 2.0, y: 2.0 },
        end: Point { x: 2.0, y: 2.0 },
    };

    let bb = line.bounding_box();

    assert_eq!(bb.min.x, 2.0);
    assert_eq!(bb.max.x, 2.0);
}
