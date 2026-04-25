use ferrumcad::geometry::{Line, Point, Geometry, Transform};

#[test]
fn test_line_length() {
    let line = Line {
        start: Point { x: 0.0, y: 0.0 },
        end:   Point { x: 3.0, y: 4.0 },
    };

    assert_eq!(line.length(), 5.0);
}

#[test]
fn test_line_midpoint() {
    let line = Line {
        start: Point { x: 0.0, y: 0.0 },
        end:   Point { x: 10.0, y: 10.0 },
    };

    let mid = line.midpoint();

    assert_eq!(mid.x, 5.0);
    assert_eq!(mid.y, 5.0);
}

#[test]
fn test_line_bounding_box() {
    let line = Line {
        start: Point { x: 5.0, y: 1.0 },
        end:   Point { x: 2.0, y: 8.0 },
    };

    let bb = line.bounding_box();

    assert_eq!(bb.min.x, 2.0);
    assert_eq!(bb.min.y, 1.0);
    assert_eq!(bb.max.x, 5.0);
    assert_eq!(bb.max.y, 8.0);
}

#[test]
fn test_line_translate() {
    let line = Line {
        start: Point { x: 0.0, y: 0.0 },
        end:   Point { x: 1.0, y: 1.0 },
    };

    let moved = line.translate(2.0, 3.0);

    assert_eq!(moved.start.x, 2.0);
    assert_eq!(moved.start.y, 3.0);
}

#[test]
fn test_segments_intersect() {
    let l1 = Line {
        start: Point { x: 0.0, y: 0.0 },
        end:   Point { x: 10.0, y: 10.0 },
    };

    let l2 = Line {
        start: Point { x: 0.0, y: 10.0 },
        end:   Point { x: 10.0, y: 0.0 },
    };

    assert!(l1.intersects(&l2));
}

#[test]
fn test_segments_do_not_intersect() {
    let l1 = Line {
        start: Point { x: 0.0, y: 0.0 },
        end:   Point { x: 5.0, y: 0.0 },
    };
    let l2 = Line {
        start: Point { x: 0.0, y: 1.0 },
        end:   Point { x: 5.0, y: 1.0 },
    };

    assert!(!l1.intersects(&l2));
}

#[test]
fn test_colinear_intersection() {
    let l1 = Line {
        start: Point { x: 0.0, y: 0.0 },
        end:   Point { x: 10.0, y: 0.0 },
    };
    let l2 = Line {
        start: Point { x: 5.0, y: 0.0},
        end:   Point { x: 15.0, y: 0.0},
    };

    assert!(l1.intersects(&l2));
}

#[test]
fn test_touching_segments() {
    let l1 = Line {
        start: Point { x: 0.0, y: 0.0 },
        end:   Point { x: 5.0, y: 5.0 },
    };
    let l2 = Line {
        start: Point { x: 5.0, y: 5.0 },
        end:   Point { x: 10.0, y: 0.0 },
    };

    assert!(l1.intersects(&l2));
}

#[test]
fn test_overlap() {
    let l1 = Line {
        start: Point { x: 0.0, y: 0.0 },
        end:   Point { x: 10.0, y: 0.0 },
    };
    let l2 = Line {
        start: Point { x: 5.0, y: 0.0 },
        end:   Point { x: 15.0, y: 0.0 },
    };

    match l1.intersection(&l2) {
        Intersection::Overlap(_) => {}
        _ => panic!("Expected overlap"),
    }
}
