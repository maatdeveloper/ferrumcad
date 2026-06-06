/* standard library */

/* external crates */

/* ferrumcad crates */
use ferrumcad::geometry::{Circle, Geometry, Point, Transform};

#[test]
fn test_circle_bounding_box() {
    let circle = Circle {
        center: Point { x: 5.0, y: 5.0 },
        radius: 2.0,
        diameter: 4.0,
    };

    let bb = circle.bounding_box();

    assert_eq!(bb.min.x, 3.0);
    assert_eq!(bb.min.y, 3.0);
    assert_eq!(bb.max.x, 7.0);
    assert_eq!(bb.max.y, 7.0);
}

#[test]
fn test_circle_scale() {
    let circle = Circle {
        center: Point { x: 1.0, y: 1.0 },
        radius: 2.0,
        diameter: 4.0,
    };

    let scaled = circle.scale(2.0);

    assert_eq!(scaled.diameter, 8.0);
}
