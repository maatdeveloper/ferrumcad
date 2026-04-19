use ferrumcad::geometry::Point;

#[test]
fn test_point_distance() {
    let p1 = Point { x: 0.0, y: 0.0 };
    let p2 = Point { x: 3.0, y: 4.0 };

    let dist = p1.distance(&p2);

    assert_eq!(dist, 5.0);
}

#[test]
fn test_point_translate() {
    let p = Point { x: 1.0, y: 2.0 };

    let moved = p.translate(3.0, -1.0);

    assert_eq!(moved.x, 4.0);
    assert_eq!(moved.y, 1.0);
}

#[test]
fn test_point_scale() {
    let p = Point { x: 2.0, y: 3.0 };

    let scaled = p.scale(2.0);

    assert_eq!(scaled.x, 4.0);
    assert_eq!(scaled.y, 6.0);
}
