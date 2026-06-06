/* standard library */

/* external crates */

/* ferrumcad crates */
use ferrumcad::cad::Shape;
use ferrumcad::geometry::{Line, Point, Transform};

#[test]
fn test_shape_translate() {
    let shape = Shape::Line(Line {
        start: Point { x: 0.0, y: 0.0 },
        end: Point { x: 1.0, y: 1.0 },
    });

    let moved = shape.translate(2.0, 3.0);

    match moved {
        Shape::Line(l) => {
            assert_eq!(l.start.x, 2.0);
            assert_eq!(l.start.y, 3.0);
        }
        _ => panic!("Expected line"),
    }
}

#[test]
fn test_shape_scaled() {
    let shape = Shape::Line(Line {
        start: Point { x: 1.0, y: 1.0 },
        end: Point { x: 4.0, y: 5.0 },
    });

    let scaled = shape.scale(2.0);

    match scaled {
        Shape::Line(l) => {
            assert_eq!(l.start.x, 2.0);
            assert_eq!(l.start.y, 2.0);
        }
        _ => panic!("Expected line"),
    }
}
