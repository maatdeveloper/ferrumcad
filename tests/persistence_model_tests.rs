/* standard library */

/* external crates */

/* ferrumcad crates */
use ferrumcad::cad::{Model, Shape};
use ferrumcad::geometry::{Line, Point};

#[test]
fn test_save_and_load_model() {
    let mut model = Model::new();

    model.shapes.push(Shape::Line(Line {
        start: Point { x: 0.0, y: 0.0 },
        end:   Point { x: 1.0, y: 1.0 },
    }));

    model.save("test.json").unwrap();

    let loaded = Model::load("test.json").unwrap();
    
    assert_eq!(loaded.shapes.len(), 1);
}