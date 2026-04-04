use ferrumcad::cad::model::Model;
use ferrumcad::geometry::Point;
use ferrumcad::cam::gcode::generate_gcode;

fn main() {
    let mut model = Model::new();

    model.add_line(
        Point { x: 0.0, y: 0.0 },
        Point { x: 10.0, y: 10.0},
    );

    let gcode = generate_gcode(&model.shapes);

    println!("{}", gcode);
}
